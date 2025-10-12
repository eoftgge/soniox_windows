#![windows_subsystem = "windows"]

use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use log4rs::config::Root;
use screen_size::get_primary_screen_size;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::types::settings::SettingsApp;
use soniox_windows::initialize_app;

const WINDOW_HEIGHT: f32 = 250.;
const OFFSET_WIDTH: f32 = 100.;

fn get_position_application(height: u64) -> (f32, f32) {
    let window_height = WINDOW_HEIGHT;
    let pos_x = OFFSET_WIDTH;
    let pos_y = height as f32 - window_height - 100.;

    (pos_x, pos_y)
}

#[tokio::main]
async fn main() -> Result<(), SonioxWindowsErrors> {
    let settings = SettingsApp::new("soniox.toml")?;
    let app = initialize_app(settings)?;
    let (width, height) = get_primary_screen_size().expect("Failed to get primary screen size");
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_icon(
                from_png_bytes(include_bytes!("../assets/icon.png")).expect("Failed to load icon"),
            )
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
        "Subtitle Live",
        native_options,
        Box::new(move |_| Ok(Box::new(app))),
    )?;
    Ok(())
}
