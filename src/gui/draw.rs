use eframe::egui::{self, Color32, RichText, Ui, Frame, Rounding};
use crate::soniox::store::TranscriptionStore;

struct VisualReplica {
    speaker: Option<String>,
    elements: Vec<TextElement>,
}

struct TextElement {
    text: String,
    is_interim: bool,
}

fn get_interim_color(main_color: Color32) -> Color32 {
    if main_color == Color32::WHITE {
        Color32::from_gray(180)
    } else if main_color == Color32::YELLOW {
        Color32::from_rgb(200, 180, 100)
    } else {
        Color32::from_white_alpha(150)
    }
}

pub fn draw_subtitles(
    ui: &mut Ui,
    store: &TranscriptionStore,
    font_size: f32,
    text_color: Color32,
) {
    let mut replicas: Vec<VisualReplica> = Vec::new();
    let all_blocks = store.blocks.iter().chain(store.interim_blocks.iter());

    for block in all_blocks {
        if block.final_text.is_empty() && block.interim_text.is_empty() { continue; }

        let speaker = block.speaker.clone();

        let should_merge = if let Some(last) = replicas.last() {
            last.speaker == speaker
        } else {
            false
        };

        if should_merge {
            let last_replica = replicas.last_mut().unwrap();
            if !block.final_text.is_empty() {
                last_replica.elements.push(TextElement { text: block.final_text.clone(), is_interim: false });
            }
            if !block.interim_text.is_empty() {
                last_replica.elements.push(TextElement { text: block.interim_text.clone(), is_interim: true });
            }
        } else {
            let mut elements = Vec::new();
            if !block.final_text.is_empty() {
                elements.push(TextElement { text: block.final_text.clone(), is_interim: false });
            }
            if !block.interim_text.is_empty() {
                elements.push(TextElement { text: block.interim_text.clone(), is_interim: true });
            }
            replicas.push(VisualReplica { speaker, elements });
        }
    }

    if replicas.is_empty() { return; }

    let max_visual_replicas = store.max_blocks();
    let total_count = replicas.len();
    let start_index = total_count.saturating_sub(max_visual_replicas);

    let screen_width = ui.ctx().content_rect().width();
    let max_width = (screen_width * 0.8).min(1200.0);
    let interim_color = get_interim_color(text_color);

    Frame::new()
        .fill(Color32::from_black_alpha(200))
        .corner_radius(12.0)
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.set_max_width(max_width);

            ui.vertical(|ui| {
                for replica in replicas.iter().skip(start_index) {

                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        if let Some(id) = &replica.speaker {
                            ui.label(RichText::new(format!("{}: ", id))
                                .size(font_size)
                                .color(text_color)
                                .strong());
                        }

                        for elem in replica.elements.iter() {
                            let color = if elem.is_interim { interim_color } else { text_color };
                            ui.label(RichText::new(&elem.text)
                                .size(font_size)
                                .strong()
                                .color(color));
                        }
                    });

                    ui.add_space(4.0);
                }
            });
        });
}