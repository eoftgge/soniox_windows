#![windows_subsystem = "windows"]

use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::initialize_app;
use soniox_windows::types::settings::SettingsApp;
use soniox_windows::windows::utils::{get_screen_size, show_error};

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

#[tokio::main]
async fn main() -> Result<(), SonioxWindowsErrors> {
    if let Err(err) = run().await {
        show_error(&format!("{}", err));
        std::process::exit(1);
    }
    Ok(())
}
