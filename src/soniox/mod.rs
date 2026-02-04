pub(crate) mod request;
pub(crate) mod transcription;
pub mod stream;

pub const URL: &str = "wss://stt-rt.soniox.com/transcribe-websocket";
pub const MODEL: &str = "stt-rt-v3";
