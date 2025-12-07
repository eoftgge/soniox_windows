use eframe::epaint::{Color32, FontId};
use eframe::egui::Ui;

pub(crate) fn trim_text_to_fit_precise(
    text: String,
    ui: &Ui,
    font_id: &FontId,
    max_width_ratio: f32,
) -> String {
    let available_width = ui.ctx().content_rect().width() * max_width_ratio;
    let mut chars: Vec<char> = text.chars().collect();
    let mut trimmed = text.to_owned();

    loop {
        let galley = ui
            .painter()
            .layout_no_wrap(trimmed.clone(), font_id.clone(), Color32::WHITE);
        let text_width = galley.size().x;

        if text_width <= available_width || chars.len() <= 4 {
            break;
        }

        chars.remove(0);
        trimmed = format!("...{}", chars.iter().collect::<String>().trim_start());
    }

    trimmed
}

#[inline]
pub(crate) fn modify_text(text: &str) -> String {
    text.replace("--", "—")
}
