use crate::errors::SonioxLiveErrors;
use crate::types::audio::AudioSample;

#[derive(Debug)]
pub enum AppEvents {
    Audio(AudioSample),
    Recycle(AudioSample),
    Error(SonioxLiveErrors)
}