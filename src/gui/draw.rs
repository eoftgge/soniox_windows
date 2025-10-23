use crate::gui::text::{modify_text, trim_text_to_fit_precise};
use crate::types::audio::AudioSubtitle;
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontId, vec2};
use egui::Ui;

pub(crate) fn draw_text_with_shadow(ui: &mut Ui, subtitle: &AudioSubtitle, font_size: f32) {
    let text = match subtitle {
        AudioSubtitle::Text(text) => modify_text(text),
        AudioSubtitle::Speaker(speaker, text) => format!("{} >> {}", speaker, modify_text(text)),
        AudioSubtitle::Empty => return,
    };

    let font = FontId::proportional(font_size);
    let trimmed = trim_text_to_fit_precise(text, ui, &font, 0.8);
    let outline_color = Color32::BLACK;
    let text_color = Color32::YELLOW;
    let thickness = 2.0;

    let painter = ui.painter();
    let rect = ui.ctx().content_rect();
    let pos = rect.left_bottom() + vec2(10., -40.);
    let offsets = [
        vec2(-thickness, 0.0),
        vec2(thickness, 0.0),
        vec2(0.0, -thickness),
        vec2(0.0, thickness),
        vec2(-thickness, -thickness),
        vec2(-thickness, thickness),
        vec2(thickness, -thickness),
        vec2(thickness, thickness),
    ];

    for offset in offsets {
        painter.text(
            pos + offset,
            Align2::LEFT_BOTTOM,
            &trimmed,
            font.clone(),
            outline_color,
        );
    }
    painter.text(pos, Align2::LEFT_BOTTOM, &trimmed, font, text_color);
}
