#![windows_subsystem = "windows"]

use std::sync::Arc;
use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use egui::{FontData, FontDefinitions};
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::initialize_app;
use soniox_windows::types::settings::SettingsApp;
use soniox_windows::windows::utils::{get_screen_size, show_error};

const FONT_BYTES: &[u8] = include_bytes!("../assets/MPLUSRounded1c-Medium.ttf");
const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");
const WINDOW_HEIGHT: f32 = 250.;
const OFFSET_WIDTH: f32 = 100.;

fn get_position_application(height: usize) -> (f32, f32) {
    let window_height = WINDOW_HEIGHT;
    let pos_x = OFFSET_WIDTH;
    let pos_y = height as f32 - window_height - 100.;

    (pos_x, pos_y)
}

async fn run() -> Result<(), SonioxWindowsErrors> {
    let settings = SettingsApp::new("soniox.toml")?;
    let app = initialize_app(settings)?;
    let (width, height) = get_screen_size();
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_icon(from_png_bytes(ICON_BYTES).expect("Failed to load icon"))
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true)
            .with_min_inner_size([width as f32 - OFFSET_WIDTH * 2., WINDOW_HEIGHT])
            .with_inner_size([width as f32 - OFFSET_WIDTH * 2., WINDOW_HEIGHT])
            .with_max_inner_size([width as f32 - OFFSET_WIDTH * 2., WINDOW_HEIGHT])
            .with_position(get_position_application(height)),
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
                Arc::new(FontData::from_static(FONT_BYTES))
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "mplus".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .push("mplus".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), SonioxWindowsErrors> {
    if let Err(err) = run().await {
        show_error(&format!("{}", err));
        std::process::exit(1);
    }
    Ok(())
}
