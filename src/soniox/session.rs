use crate::errors::SonioxLiveErrors;
use crate::soniox::WsStream;
use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use tungstenite::{Bytes, Message, Utf8Bytes};

pub struct SonioxSessionReader(pub(super) SplitStream<WsStream>);
pub struct SonioxSessionWriter(pub(super) SplitSink<WsStream, Message>);

impl SonioxSessionReader {
    pub async fn recv_message(&mut self) -> Result<Message, SonioxLiveErrors> {
        match self.0.next().await {
            Some(Ok(msg)) => Ok(msg),
            Some(Err(e)) => Err(e.into()),
            None => Err(SonioxLiveErrors::ConnectionLost)
        }
    }
}

impl SonioxSessionWriter {
    pub async fn send_pong(&mut self, data: Bytes) -> Result<(), SonioxLiveErrors> {
        tracing::debug!("Sending pong");
        self.0.send(Message::Pong(data)).await?;
        Ok(())
    }

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