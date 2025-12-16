use crate::types::audio::AudioSubtitle;
use crate::types::soniox::SonioxTranscriptionResponse;

pub struct TranscriptionState {
    finishes_lines: Vec<AudioSubtitle>,
    interim_line: AudioSubtitle,
    max_lines: usize,
}

impl TranscriptionState {
    pub fn new(max_lines: usize) -> Self {
        assert!(max_lines > 0);

        Self {
            finishes_lines: Vec::new(),
            interim_line: AudioSubtitle::default(),
            max_lines,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &AudioSubtitle> {
        self.finishes_lines
            .iter()
            .chain(std::iter::once(&self.interim_line))
            .rev()
    }

    pub fn handle_transcription(&mut self, response: SonioxTranscriptionResponse) {
        let mut interim_text = String::new();
        let mut interim_speaker = Option::<String>::None;
        let mut final_text = String::new();
        let mut final_speaker = Option::<String>::None;

        for token in response.tokens {
            log::debug!("Token from WS: {:?}", token);

            if token.translation_status.as_deref() == Some("original") {
                continue;
            } else if token.is_final {
                if final_speaker != token.speaker {
                    self.push_final(final_speaker, final_text);
                    final_speaker = token.speaker;
                    final_text = String::new();
                }

                final_text.push_str(&token.text);

            } else {
                if interim_speaker != token.speaker {
                    interim_speaker = token.speaker;
                    interim_text = String::new();
                }

                interim_text.push_str(&token.text);
            }
        }

        self.push_final(final_speaker, final_text);
        self.update_interim(interim_speaker, interim_text);
    }

    fn push_final(&mut self, speaker: Option<String>, text: String) {
        match (self.finishes_lines.last_mut(), speaker) {
            (Some(last), current)
                if last.speaker == current
            => {
                last.text.push_str(&text);
            }
            (_, s) => self.finishes_lines.push(AudioSubtitle::new(s, text))
        }

        if self.finishes_lines.len() > self.max_lines - 1 {
            self.finishes_lines.remove(0);
        }
    }

    fn update_interim(&mut self, speaker: Option<String>, text: String) {
        if text.is_empty() {
            return;
        }

        self.interim_line = AudioSubtitle::new(speaker, text);
    }
}
