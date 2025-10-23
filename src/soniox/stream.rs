use crate::errors::SonioxWindowsErrors;
use crate::types::audio::{AudioMessage, AudioSubtitle};
use crate::types::settings::SettingsApp;
use crate::types::soniox::SonioxTranscriptionResponse;
use crate::soniox::render::render_transcription;
use crate::soniox::request::create_request;
use crate::soniox::URL;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::{Bytes, Message, Utf8Bytes};

async fn listen_soniox_stream(
    bytes: Vec<u8>,
    tx_subs: UnboundedSender<AudioSubtitle>,
    mut rx_audio: UnboundedReceiver<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    'stream: loop {
        let url = URL.into_client_request()?;
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();
        write
            .send(Message::Text(Utf8Bytes::try_from(bytes.clone())?))
            .await?;

        let tx_subs = tx_subs.clone();
        let reader = async move {
            while let Some(msg) = read.next().await {
                if let Message::Text(txt) = msg? {
                    let response: SonioxTranscriptionResponse = serde_json::from_str(&txt)?;
                    let subtitle = render_transcription(&response);
                    let _ = tx_subs.send(subtitle);
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            <Result<(), SonioxWindowsErrors>>::Ok(())
        };

        tokio::spawn(async move {
            if let Err(err) = reader.await {
                log::error!("error during read message {}", err);
            }
        });
        while let Some(message) = rx_audio.recv().await {
            match message {
                AudioMessage::Audio(buffer) => {
                    if buffer.is_empty() {
                        break;
                    }
                    let pcm16: Vec<u8> = buffer
                        .iter()
                        .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                        .flat_map(|s| s.to_le_bytes())
                        .collect();

                    let result = write.send(Message::Binary(Bytes::from(pcm16))).await;

                    if let Err(err) = result {
                        log::error!("error during sent binary -> {:?}", err);
                        continue 'stream;
                    }
                }
                AudioMessage::Stop => {
                    let _ = write.send(Message::Binary(Bytes::new())).await;
                    break 'stream;
                }
            }
        }

        let result = write.send(Message::Binary(Bytes::new())).await;
        if let Err(err) = result {
            log::error!("error during sent empty binary -> {:?}", err);
            break;
        }
    }

    Ok(())
}

pub async fn start_soniox_stream(
    settings: SettingsApp,
    tx_subs: UnboundedSender<AudioSubtitle>,
    rx_audio: UnboundedReceiver<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    let request = create_request(settings);
    let bytes = serde_json::to_vec(&request)?;

    log::info!("Started Soniox stream!");
    log::info!("Starting to listen websocket stream Soniox...");
    listen_soniox_stream(bytes, tx_subs, rx_audio).await
}
