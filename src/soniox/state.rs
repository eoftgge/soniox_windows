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
            current_line: AudioSubtitle::default(),
            max_lines,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &AudioSubtitle> {
        self.finishes_lines
            .iter()
            .chain(std::iter::once(&self.current_line))
            .rev()
    }

    pub fn handle_transcription(&mut self, response: SonioxTranscriptionResponse) {
        let mut current_text = String::new();
        let mut current_speaker = Option::<String>::None;
        let mut is_final = false;

        for token in response.tokens {
            log::debug!("Token from WS: {:?}", token);

            if token.translation_status.as_deref() == Some("original") {
                continue;
            }

            match (token.speaker, &current_speaker) {
                (Some(last), Some(current)) if &last != current => {
                    self.push(current_speaker, current_text, is_final);
                    current_speaker = Some(last);
                    current_text = String::new();
                },
                (Some(last), None) => {
                    current_speaker = Some(last);
                    current_text = String::new();
                },
                _ => {}
            }
            current_text.push_str(&token.text);

            is_final = token.is_final;

            if is_final {

            }
        }

        if current_text.is_empty() {
            return;
        }

        self.push(current_speaker, current_text, is_final);
    }

    fn push(&mut self, speaker: Option<String>, text: String, is_final: bool) {
        let entry = AudioSubtitle::new(speaker, text);

        if is_final {
            self.finishes_lines.push(entry);
        } else {
            self.current_line = entry;
        }

        if self.finishes_lines.len() > self.max_lines - 1 {
            self.finishes_lines.remove(0);
        }
    }
}
