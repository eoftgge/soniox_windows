use std::fmt::Display;

pub type AudioSample = Vec<f32>;

#[derive(Debug, PartialEq, Eq)]
pub struct AudioSubtitle {
    pub(crate) speaker: Option<String>,
    pub(crate) text: String,
}

#[derive(Debug)]
pub enum AudioMessage {
    Audio(AudioSample),
    Stop,
}

impl AudioSubtitle {
    pub fn new(speaker: Option<String>, text: String) -> Self {
        Self { speaker, text }
    }
}

impl Default for AudioSubtitle {
    fn default() -> Self {
        Self {
            speaker: None,
            text: "... waiting for the sound ...".to_string(),
        }
    }
}
