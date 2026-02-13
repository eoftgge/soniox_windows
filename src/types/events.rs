use crate::errors::SonioxLiveErrors;
use crate::types::soniox::SonioxTranscriptionResponse;

#[derive(Debug)]
pub enum SonioxEvent {
    Transcription(SonioxTranscriptionResponse),
    Warning(String),
    Error(SonioxLiveErrors),
    Connected(bool),
}

impl From<&str> for SonioxEvent {
    fn from(value: &str) -> Self {
        Self::Warning(value.into())
    }
}
