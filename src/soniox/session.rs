use crate::errors::SonioxLiveErrors;
use crate::soniox::WsStream;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use tungstenite::{Bytes, Message, Utf8Bytes};

// const MAX_RETRIES: u32 = 5;
// const RECONNECT_DELAY: u64 = 1000; // ms
// const ERROR_CODES_RECONNECT: &[usize] = &[408, 502, 503];

pub struct SonioxSessionReader(pub(super) SplitStream<WsStream>);
pub struct SonioxSessionWriter(pub(super) SplitSink<WsStream, Message>);

impl SonioxSessionReader {
    pub async fn next(&mut self) -> Result<Utf8Bytes, SonioxLiveErrors> {
        match self.0.next().await {
            Some(Ok(msg)) => {
                if let Message::Text(text) = msg {
                    return Ok(text);
                }
                Ok(Utf8Bytes::default())
            }
            Some(Err(e)) => Err(e.into()),
            None => Err(SonioxLiveErrors::ConnectionLost)
        }
    }
}

impl SonioxSessionWriter {
    pub async fn send_text(&mut self, data: impl Into<Utf8Bytes>) -> Result<(), SonioxLiveErrors> {
        let message = Message::text(data.into());
        self.0.send(message).await?;
        Ok(())
    }

    pub async fn send_bytes(&mut self, data: impl Into<Bytes>) -> Result<(), SonioxLiveErrors> {
        let message = Message::Binary(data.into());
        self.0.send(message).await?;
        Ok(())
    }
}