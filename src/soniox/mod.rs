use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub mod action;
pub mod session;
pub mod request;
pub mod connection;
pub mod worker;

pub const URL: &str = "wss://stt-rt.soniox.com/transcribe-websocket";
pub const MODEL: &str = "stt-rt-v4";

pub(crate) type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
