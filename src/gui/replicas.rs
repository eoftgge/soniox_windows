use crate::soniox::store::TranscriptionStore;

pub struct VisualReplica {
    pub speaker: Option<String>,
    pub elements: Vec<TextElement>,
}

pub struct TextElement {
    pub text: String,
    pub is_interim: bool,
}

impl VisualReplica {
    pub fn new(speaker: Option<String>) -> Self {
        Self {
            speaker,
            elements: Vec::new(),
        }
    }

    pub fn add_text(&mut self, text: String, is_interim: bool) {
        self.elements.push(TextElement { text, is_interim });
    }
}

pub fn prepare_replicas(store: &TranscriptionStore) -> Vec<VisualReplica> {
    let mut replicas: Vec<VisualReplica> = Vec::new();
    let all_blocks = store.blocks.iter().chain(store.interim_blocks.iter());

    for block in all_blocks {
        if block.final_text.is_empty() && block.interim_text.is_empty() {
            continue;
        }

        let speaker = block.speaker.clone();
        let should_merge = replicas
            .last()
            .map(|last| last.speaker == speaker)
            .unwrap_or(false);

        if should_merge {
            let last_replica = replicas.last_mut().unwrap();
            if !block.final_text.is_empty() {
                last_replica.add_text(block.final_text.clone(), false);
            }
            if !block.interim_text.is_empty() {
                last_replica.add_text(block.interim_text.clone(), true);
            }
        } else {
            let mut new_replica = VisualReplica::new(speaker);
            if !block.final_text.is_empty() {
                new_replica.add_text(block.final_text.clone(), false);
            }
            if !block.interim_text.is_empty() {
                new_replica.add_text(block.interim_text.clone(), true);
            }
            replicas.push(new_replica);
        }
    }

    replicas
}
