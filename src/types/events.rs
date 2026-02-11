use crate::types::soniox::SonioxTranscriptionResponse;
use crate::errors::SonioxLiveErrors;

#[derive(Debug)]
pub enum SonioxEvent {
    Transcription(SonioxTranscriptionResponse),
    Warning(&'static str),
    Error(SonioxLiveErrors)
}