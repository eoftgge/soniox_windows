use crate::types::audio::AudioSubtitle;
use crate::types::soniox::SonioxTranscriptionResponse;

pub struct TranscriptionState {
    finishes_lines: Vec<AudioSubtitle>,
    current_line: AudioSubtitle,
    max_lines: usize,
}

impl TranscriptionState {
    pub fn new(max_lines: usize) -> Self {
        assert!(max_lines > 0);
        Self {
            finishes_lines: Vec::new(),
            current_line: AudioSubtitle::Empty,
            max_lines,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item=&AudioSubtitle>  {
        self.finishes_lines.iter().chain(std::iter::once(&self.current_line)).rev()
    }

    pub fn handle_transcription(&mut self, _response: SonioxTranscriptionResponse) {

    }
}