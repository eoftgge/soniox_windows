use crate::gui::color::get_interim_color;
use crate::soniox::replicas::{VisualReplica, prepare_replicas};
use crate::transcription::store::TranscriptionStore;
use eframe::egui::{Color32, Frame, RichText, Ui};

pub fn draw_subtitles(
    ui: &mut Ui,
    store: &TranscriptionStore,
    font_size: f32,
    text_color: Color32,
    background_color: Color32,
) {
    let replicas = prepare_replicas(store);
    if replicas.is_empty() {
        return;
    }

    let max_visual_replicas = store.max_blocks();
    let total_count = replicas.len();
    let start_index = total_count.saturating_sub(max_visual_replicas);
    let visible_replicas = replicas.iter().skip(start_index);

    let screen_width = ui.ctx().content_rect().width();
    let max_width = (screen_width * 0.8).min(1200.0);
    let interim_color = get_interim_color(text_color);

    Frame::new()
        .fill(background_color)
        .corner_radius(12.0)
        .inner_margin(16.0)
        .show(ui, |ui| {
            ui.set_max_width(max_width);

            ui.vertical(|ui| {
                for replica in visible_replicas {
                    draw_replica_row(ui, replica, font_size, text_color, interim_color);

                    ui.add_space(4.0);
                }
            });
        });
}

fn draw_replica_row(
    ui: &mut Ui,
    replica: &VisualReplica,
    font_size: f32,
    text_color: Color32,
    interim_color: Color32,
) {
    ui.horizontal_wrapped(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;

        if let Some(id) = &replica.speaker {
            ui.label(
                RichText::new(format!("{}: ", id))
                    .size(font_size)
                    .color(text_color)
                    .strong(),
            );
        }

        for elem in replica.elements.iter() {
            let color = if elem.is_interim {
                interim_color
            } else {
                text_color
            };

            ui.label(
                RichText::new(&elem.text)
                    .size(font_size)
                    .strong()
                    .color(color),
            );
        }
    });
}
