use crate::errors::SonioxLiveErrors;
use crate::soniox::URL;
use crate::soniox::action::StreamAction;
use crate::types::audio::AudioSample;
use crate::types::events::SonioxEvent;
use crate::types::soniox::{SonioxTranscriptionRequest, SonioxTranscriptionResponse};
use futures_util::{SinkExt, StreamExt};
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::{Bytes, Message, Utf8Bytes};

const MAX_RETRIES: u32 = 5;
const RECONNECT_DELAY: u64 = 1000; // ms

pub struct SonioxClient {
    tx_event: Sender<SonioxEvent>,
    rx_audio: Receiver<AudioSample>,
    tx_recycle: Sender<AudioSample>,
}

impl SonioxClient {
    pub fn new(
        tx_event: Sender<SonioxEvent>,
        tx_recycle: Sender<AudioSample>,
        rx_audio: Receiver<AudioSample>,
    ) -> Self {
        Self {
            tx_event,
            rx_audio,
            tx_recycle,
        }
    }

    pub async fn start(
        &mut self,
        request: &SonioxTranscriptionRequest,
    ) -> Result<(), SonioxLiveErrors> {
        let init_bytes = serde_json::to_vec(request)?;
        let mut retry_count = 0;

        loop {
            let url = URL.into_client_request()?;
            tracing::debug!("Connecting to Soniox... (Attempt {})", retry_count + 1);

            match connect_async(url).await {
                Ok((ws_stream, _)) => {
                    retry_count = 0;
                    let action = self.run_session(ws_stream, &init_bytes).await?;

                    if let StreamAction::Stop = action {
                        return Ok(());
                    }
                }
                Err(e) => tracing::error!("Connection failed: {}", e),
            }

            sleep(Duration::from_millis(RECONNECT_DELAY)).await;
            retry_count += 1;
            if retry_count > MAX_RETRIES {
                let _ = self
                    .tx_event
                    .send(SonioxEvent::Error(SonioxLiveErrors::ConnectionLost))
                    .await;
                return Err(SonioxLiveErrors::ConnectionLost);
            }
        }
    }

    async fn run_session(
        &mut self,
        ws_stream: tokio_tungstenite::WebSocketStream<
            tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
        >,
        init_bytes: &[u8],
    ) -> Result<StreamAction, SonioxLiveErrors> {
        let (mut write, mut read) = ws_stream.split();
        write
            .send(Message::Text(Utf8Bytes::try_from(init_bytes.to_vec())?))
            .await?;

        loop {
            tokio::select! {
                audio_event = self.rx_audio.recv() => {
                    let action = self.handle_audio(audio_event, &mut write).await?;
                    if !matches!(action, StreamAction::Continue) { return Ok(action); }
                }
                ws_event = read.next() => {
                    let action = self.handle_ws(ws_event, &mut write).await;
                    if !matches!(action, StreamAction::Continue) { return Ok(action); }
                }
            }
        }
    }

    async fn handle_audio<W>(
        &mut self,
        msg: Option<AudioSample>,
        writer: &mut W,
    ) -> Result<StreamAction, SonioxLiveErrors>
    where
        W: SinkExt<Message, Error = tungstenite::Error> + Unpin,
    {
        match msg {
            Some(mut buffer) => {
                if !buffer.is_empty() {
                    let slice: &[u8] = bytemuck::cast_slice(&buffer);

                    writer
                        .send(Message::Binary(Bytes::copy_from_slice(slice)))
                        .await?;
                    buffer.clear();
                    let _ = self.tx_recycle.send(buffer).await;
                }
                Ok(StreamAction::Continue)
            }
            None => {
                tracing::debug!("Audio channel closed.");
                Ok(StreamAction::Stop)
            }
        }
    }

    async fn handle_ws<W>(
        &mut self,
        msg: Option<Result<Message, tungstenite::Error>>,
        writer: &mut W,
    ) -> StreamAction
    where
        W: SinkExt<Message, Error = tungstenite::Error> + Unpin,
    {
        match msg {
            Some(Ok(message)) => match message {
                Message::Text(txt) => {
                    let response = serde_json::from_str::<SonioxTranscriptionResponse>(&txt);
                    if let Ok(r) = response
                        && self
                            .tx_event
                            .send(SonioxEvent::Transcription(r))
                            .await
                            .is_err()
                    {
                        tracing::error!("UI channel closed");
                        return StreamAction::Stop;
                    }
                    StreamAction::Continue
                }
                Message::Ping(data) => {
                    let _ = writer.send(Message::Pong(data)).await;
                    StreamAction::Continue
                }
                Message::Close(_) => {
                    tracing::warn!("Server closed connection");
                    let _ = self
                        .tx_event
                        .send(SonioxEvent::Warning(
                            "Server closed connection. Reconnection...",
                        ))
                        .await;
                    StreamAction::Reconnect
                }
                _ => StreamAction::Continue,
            },
            Some(Err(e)) => {
                tracing::error!("WS Read Error: {}", e);
                let _ = self
                    .tx_event
                    .send(SonioxEvent::Warning("WS read error... Reconnection"))
                    .await;
                StreamAction::Reconnect
            }
            None => {
                tracing::warn!("WS Stream ended");
                let _ = self
                    .tx_event
                    .send(SonioxEvent::Warning("WS stream ended... Reconnection"))
                    .await;
                StreamAction::Reconnect
            }
        }
    }
}
