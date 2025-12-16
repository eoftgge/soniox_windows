use crate::gui::text::trim_text_to_fit_precise;
use crate::types::audio::AudioSubtitle;
use eframe::egui::{Ui, pos2};
use eframe::emath::Align2;
use eframe::epaint::{Color32, FontId, vec2};

pub(crate) fn draw_text_with_shadow<'a>(
    ui: &mut Ui,
    lines: impl Iterator<Item = &'a AudioSubtitle>,
    font_size: f32,
    text_color: Color32,
) {
    let font = FontId::proportional(font_size);
    let painter = ui.painter();
    let rect = ui.ctx().content_rect();
    let outline_color = Color32::BLACK;
    let thickness = 2.0;
    let mut y = rect.bottom() - 10.0;

    for line in lines {
        let mut text = String::new();
        if let Some(speaker) = line.speaker.to_owned() {
            text.push_str(&(speaker + " >> "));
        }
        let trimmed = trim_text_to_fit_precise(&line.text, ui, &font, 0.8);
        text.push_str(&trimmed);

        let pos = pos2(rect.left() + 10., y);
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
        painter.text(pos, Align2::LEFT_BOTTOM, &trimmed, font.clone(), text_color);

        y -= font_size * 1.2;
    }
}
