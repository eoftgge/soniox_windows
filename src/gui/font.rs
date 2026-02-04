use eframe::egui::{Context, FontData, FontDefinitions, FontFamily};
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("../../assets/MPLUSRounded1c-Medium.ttf");

pub fn setup_custom_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "mplus".to_owned(),
        Arc::new(FontData::from_static(FONT_BYTES)),
    );
    fonts
        .families
        .entry(FontFamily::Proportional)
        .or_default()
        .insert(0, "mplus".to_owned());
    fonts
        .families
        .entry(FontFamily::Monospace)
        .or_default()
        .push("mplus".to_owned());
    ctx.set_fonts(fonts);
}
