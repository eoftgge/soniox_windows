use crate::types::soniox::SonioxTranscriptionResponse;
use crate::errors::SonioxLiveErrors;

#[derive(Debug)]
pub enum AppEvent {
    Transcription(SonioxTranscriptionResponse),
    Error(SonioxLiveErrors)
}