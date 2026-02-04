use crate::errors::SonioxWindowsErrors;
use crate::soniox::request::create_request;
use crate::soniox::URL;
use crate::types::audio::AudioMessage;
use crate::types::settings::SettingsApp;
use crate::types::soniox::SonioxTranscriptionResponse;
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::{Bytes, Message, Utf8Bytes};

const MAX_RETRIES: u32 = 5;
const RECONNECT_DELAY: u64 = 1000; // ms

enum StreamAction {
    Continue,
    Reconnect,
    Stop,
}

fn convert_audio_chunk(buffer: Vec<f32>) -> Vec<u8> {
    let mut pcm16 = Vec::with_capacity(buffer.len() * 2);
    for s in buffer {
        let sample = (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16;
        pcm16.extend_from_slice(&sample.to_le_bytes());
    }
    pcm16
}

async fn handle_audio_message<W>(
    msg: Option<AudioMessage>,
    writer: &mut W,
) -> Result<StreamAction, W::Error>
where
    W: SinkExt<Message> + Unpin,
{
    match msg {
        Some(AudioMessage::Audio(buffer)) => {
            if !buffer.is_empty() {
                let binary = convert_audio_chunk(buffer);
                writer.send(Message::Binary(Bytes::from(binary))).await?;
            }
            Ok(StreamAction::Continue)
        }
        Some(AudioMessage::Stop) => {
            log::info!("Stop command received.");
            let _ = writer.send(Message::Binary(Bytes::new())).await;
            let _ = writer.close().await;
            Ok(StreamAction::Stop)
        }
        None => {
            log::info!("Audio channel closed.");
            Ok(StreamAction::Stop)
        }
    }
}

async fn handle_ws_message(
    msg: Option<Result<Message, tungstenite::Error>>,
    tx_ui: &UnboundedSender<SonioxTranscriptionResponse>,
    writer: &mut (impl SinkExt<Message, Error = tungstenite::Error> + Unpin),
) -> StreamAction {
    match msg {
        Some(Ok(message)) => match message {
            Message::Text(txt) => {
                let response = serde_json::from_str::<SonioxTranscriptionResponse>(&txt);
                if let Ok(r) = response && tx_ui.send(r).is_err() {
                    log::error!("UI channel closed");
                    return StreamAction::Stop;
                }
                StreamAction::Continue
            }
            Message::Ping(data) => {
                let _ = writer.send(Message::Pong(data)).await;
                StreamAction::Continue
            }
            Message::Close(_) => {
                log::warn!("Server closed connection");
                StreamAction::Reconnect
            }
            _ => StreamAction::Continue,
        },
        Some(Err(e)) => {
            log::error!("WS Read Error: {}", e);
            StreamAction::Reconnect
        }
        None => {
            log::warn!("WS Stream ended");
            StreamAction::Reconnect
        }
    }
}

async fn run_active_session(
    ws_stream: tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
    tx_ui: UnboundedSender<SonioxTranscriptionResponse>,
    rx_audio: &mut UnboundedReceiver<AudioMessage>,
    init_bytes: &[u8],
) -> Result<StreamAction, SonioxWindowsErrors> {
    let (mut write, mut read) = ws_stream.split();

    if let Err(e) = write.send(Message::Text(Utf8Bytes::try_from(init_bytes.to_vec())?)).await {
        log::error!("Init handshake failed: {}", e);
        return Ok(StreamAction::Reconnect);
    }

    loop {
        tokio::select! {
            audio_event = rx_audio.recv() => {
                match handle_audio_message(audio_event, &mut write).await {
                    Ok(StreamAction::Continue) => continue,
                    Ok(action) => return Ok(action),
                    Err(e) => {
                        log::error!("WS Write Error: {}", e);
                        return Ok(StreamAction::Reconnect);
                    }
                }
            }

            ws_event = read.next() => {
                match handle_ws_message(ws_event, &tx_ui, &mut write).await {
                    StreamAction::Continue => continue,
                    action => return Ok(action),
                }
            }
        }
    }
}

async fn listen_soniox_stream(
    init_bytes: Vec<u8>,
    tx_transcription: UnboundedSender<SonioxTranscriptionResponse>,
    mut rx_audio: UnboundedReceiver<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    let mut retry_count = 0;

    loop {
        let url = URL.into_client_request().map_err(|_| SonioxWindowsErrors::WssConnectionError)?;

        log::info!("Connecting... (Attempt {})", retry_count + 1);

        match connect_async(url).await {
            Ok((ws_stream, _)) => {
                log::info!("Connected!");
                retry_count = 0;

                let action = run_active_session(
                    ws_stream,
                    tx_transcription.clone(),
                    &mut rx_audio,
                    &init_bytes
                ).await?;

                match action {
                    StreamAction::Stop => {
                        log::info!("Stream finished normally.");
                        return Ok(());
                    }
                    StreamAction::Reconnect => {
                        log::warn!("Session lost. Reconnecting...");
                    }
                    StreamAction::Continue => unreachable!(),
                }
            }
            Err(e) => {
                log::error!("Connection failed: {}", e);
            }
        }

        sleep(Duration::from_millis(RECONNECT_DELAY)).await;
        retry_count += 1;

        if retry_count > MAX_RETRIES {
            return Err(SonioxWindowsErrors::WssConnectionError);
        }
    }
}

pub async fn start_soniox_stream(
    settings: &SettingsApp,
    tx_transcription: UnboundedSender<SonioxTranscriptionResponse>,
    rx_audio: UnboundedReceiver<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    let request = create_request(settings)?;
    let bytes = serde_json::to_vec(&request)?;

    log::info!("Started Soniox stream!");
    log::info!("Starting to listen websocket stream Soniox...");
    listen_soniox_stream(bytes, tx_transcription, rx_audio).await
}
