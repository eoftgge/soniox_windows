#![windows_subsystem = "windows"]

use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::gui::font::setup_custom_fonts;
use soniox_windows::settings::SettingsApp;
use soniox_windows::windows::utils::show_error;
use soniox_windows::{ICON_BYTES, initialize_app};

async fn run() -> Result<(), SonioxWindowsErrors> {
    let settings = SettingsApp::new("soniox.toml")?;
    let app = initialize_app(settings)?;
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_icon(from_png_bytes(ICON_BYTES).expect("Failed to load icon"))
            .with_inner_size([400., 600.])
            .with_resizable(false)
            .with_decorations(true)
            .with_always_on_top()
            .with_transparent(true)
            .with_maximize_button(false),
        ..Default::default()
    };

    tracing::info!("Starting application");
    eframe::run_native(
        "Subtitles Live",
        native_options,
        Box::new(move |cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(app))
        }),
    )?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        show_error(&format!("{}", err));
        tracing::error!("error in soniox_windows!: {:?}", err);
        std::process::exit(1);
    }
}
