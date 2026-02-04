#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct SubtitleBlock {
    pub(crate) speaker: Option<String>,
    pub(crate) final_text: String,
    pub(crate) interim_text: String,
}

impl SubtitleBlock {
    pub fn new(speaker: Option<String>) -> Self {
        Self {
            speaker,
            ..Default::default()
        }
    }
}
