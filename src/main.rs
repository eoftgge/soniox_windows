#![windows_subsystem = "windows"]

use eframe::egui::ViewportBuilder;
use eframe::egui::{FontData, FontDefinitions, FontFamily};
use eframe::icon_data::from_png_bytes;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::initialize_app;
use soniox_windows::types::settings::SettingsApp;
use soniox_windows::windows::utils::show_error;
use std::sync::Arc;

const FONT_BYTES: &[u8] = include_bytes!("../assets/MPLUSRounded1c-Medium.ttf");
const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

async fn run() -> Result<(), SonioxWindowsErrors> {
    let settings = SettingsApp::new("soniox.toml")?;
    let app = initialize_app(settings)?;
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_icon(from_png_bytes(ICON_BYTES).expect("Failed to load icon"))
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true)
            .with_maximized(true),
        ..Default::default()
    };

    log::info!("Starting application");
    eframe::run_native(
        "Subtitles Live",
        native_options,
        Box::new(move |cc| {
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
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        show_error(&format!("{}", err));
        log::error!("error in soniox_windows!: {:?}", err);
        std::process::exit(1);
    }
}
