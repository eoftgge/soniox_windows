use crate::types::soniox::SonioxTranscriptionResponse;
use crate::types::subtitles::SubtitleBlock;
use std::collections::VecDeque;
use std::time::Duration;
use tokio::time::Instant;

pub struct TranscriptionStore {
    pub blocks: VecDeque<SubtitleBlock>,
    pub interim_blocks: Vec<SubtitleBlock>,
    max_blocks: usize,
    last_activity: Instant,
}

impl TranscriptionStore {
    pub fn new(max_blocks: usize) -> Self {
        Self {
            blocks: VecDeque::with_capacity(max_blocks),
            interim_blocks: Vec::with_capacity(max_blocks),
            last_activity: Instant::now(),
            max_blocks,
        }
    }

    pub fn max_blocks(&self) -> usize {
        self.max_blocks
    }

    pub fn update(&mut self, response: SonioxTranscriptionResponse) {
        if !response.tokens.is_empty() {
            self.last_activity = Instant::now();
        }

        for token in &response.tokens {
            if token.translation_status.as_deref() == Some("original") {
                continue;
            }

            if token.is_final {
                let speaker = token.speaker.clone();
                let needs_new = match self.blocks.back() {
                    Some(last) => last.speaker != speaker || last.final_text.len() > 200,
                    None => true,
                };

                if needs_new {
                    self.blocks.push_back(SubtitleBlock::new(speaker.clone()));
                    if self.blocks.len() > self.max_blocks {
                        self.blocks.pop_front();
                    }
                }

                if let Some(block) = self.blocks.back_mut() {
                    block.final_text.push_str(&token.text);
                }
            }
        }

        self.interim_blocks.clear();
        let mut current_interim_block: Option<SubtitleBlock> = None;

        for token in &response.tokens {
            if !token.is_final && token.translation_status.as_deref() != Some("original") {
                let speaker = token.speaker.clone();
                let speaker_changed = match &current_interim_block {
                    Some(block) => block.speaker != speaker,
                    None => true,
                };

                if speaker_changed {
                    if let Some(block) = current_interim_block.take() {
                        self.interim_blocks.push(block);
                    }
                    let mut new_block = SubtitleBlock::new(speaker);
                    new_block.interim_text.push_str(&token.text);
                    current_interim_block = Some(new_block);
                } else if let Some(block) = &mut current_interim_block {
                    block.interim_text.push_str(&token.text);
                }
            }
        }

        if let Some(block) = current_interim_block {
            self.interim_blocks.push(block);
        }
    }

    pub fn clear_if_silent(&mut self, timeout: Duration) {
        if self.last_activity.elapsed() > timeout
            && (!self.blocks.is_empty()
            || !self.interim_blocks.is_empty()) {
            self.blocks.clear();
            self.interim_blocks.clear();
        }
    }
}
