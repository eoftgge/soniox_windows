pub type AudioSample = Vec<f32>;

#[derive(Debug, PartialEq, Eq)]
pub enum AudioSubtitle {
    Text(String),
    Speaker(String, String),
    Empty,
}

#[derive(Debug)]
pub enum AudioMessage {
    Audio(AudioSample),
    Stop,
}

impl AudioSubtitle {

    pub fn is_empty(&self) -> bool {
        AudioSubtitle::Empty == *self
    }
}

impl Default for AudioSubtitle {
    fn default() -> Self {
        Self::Text("... waiting for the sound ...".into())
    }
}