#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AudioSubtitle {
    pub(crate) speaker: Option<String>,
    pub(crate) text: String,
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