use crate::errors::SonioxLiveErrors;
use crate::types::soniox::SonioxTranscriptionResponse;

#[derive(Debug)]
pub enum SonioxEvent {
    Transcription(SonioxTranscriptionResponse),
    Warning(&'static str),
    Error(SonioxLiveErrors),
}
