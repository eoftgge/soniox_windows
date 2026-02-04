pub(crate) mod request;
pub mod stream;
pub(crate) mod transcription;

pub const URL: &str = "wss://stt-rt.soniox.com/transcribe-websocket";
pub const MODEL: &str = "stt-rt-v3";
