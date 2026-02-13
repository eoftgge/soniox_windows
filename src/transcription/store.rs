use crate::types::soniox::SonioxTranscriptionResponse;
use crate::types::subtitles::SubtitleBlock;
use eframe::egui::Context;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct TranscriptionStore {
    pub blocks: VecDeque<SubtitleBlock>,
    pub interim_blocks: Vec<SubtitleBlock>,
    max_blocks: usize,
    last_activity: Option<Instant>,
}

impl TranscriptionStore {
    pub fn new(max_blocks: usize) -> Self {
        Self {
            blocks: VecDeque::with_capacity(max_blocks),
            interim_blocks: Vec::with_capacity(max_blocks),
            max_blocks,
            last_activity: None,
        }
    }

    pub fn max_blocks(&self) -> usize {
        self.max_blocks
    }

    pub fn update(&mut self, response: SonioxTranscriptionResponse) {
        self.interim_blocks.clear();
        let mut current_interim_block: Option<SubtitleBlock> = None;

        for token in &response.tokens {
            tracing::debug!("{:?}", token);
            if token.translation_status.as_deref() == Some("original") {
                continue;
            }

            if token.is_final {
                let speaker = token.speaker.clone();
                let needs_new = match self.blocks.back() {
                    Some(last) => last.speaker != speaker || last.text.len() > 200,
                    None => true,
                };

                if needs_new {
                    self.blocks.push_back(SubtitleBlock::new(speaker.clone()));
                    if self.blocks.len() > self.max_blocks {
                        self.blocks.pop_front();
                    }
                }

                if let Some(block) = self.blocks.back_mut() {
                    block.text.push_str(&token.text);
                }
            } else {
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
                    new_block.text.push_str(&token.text);
                    current_interim_block = Some(new_block);
                } else if let Some(block) = &mut current_interim_block {
                    block.text.push_str(&token.text);
                }
            }
        }

        if let Some(block) = current_interim_block {
            self.interim_blocks.push(block);
        }
        if !response.tokens.is_empty() {
            self.last_activity = Some(Instant::now());
        }
    }

    pub fn resize(&mut self, new_max_blocks: usize) {
        self.max_blocks = new_max_blocks;
        while self.blocks.len() > self.max_blocks {
            self.blocks.pop_front();
        }
    }

    pub fn last_activity(&self) -> Option<Instant> {
        self.last_activity
    }

    pub fn clear_if_silent(&mut self, timeout: Duration) {
        if let Some(last_activity) = self.last_activity
            && last_activity.elapsed() >= timeout
        {
            self.blocks.clear();
            self.interim_blocks.clear();
            self.last_activity = None;
        }
    }

    pub fn schedule(&mut self, ctx: Context, timeout: Duration) {
        if let Some(last_activity) = self.last_activity() {
            let elapsed = last_activity.elapsed();
            if elapsed < timeout {
                ctx.request_repaint_after(timeout - elapsed);
            }
        }
    }
}
