#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{IconData, ViewportBuilder};
use eframe::icon_data::from_png_bytes;
use soniox_live::errors::SonioxLiveErrors;
use soniox_live::gui::font::setup_custom_fonts;
use soniox_live::settings::SettingsApp;
use soniox_live::{ICON_BYTES, initialize_app};

async fn run() -> Result<(), SonioxLiveErrors> {
    let settings = SettingsApp::new("soniox.toml")?;
    let app = initialize_app(settings)?;
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_icon(from_png_bytes(ICON_BYTES).unwrap_or_else(|_| {
                tracing::warn!("Bytes of icon is incorrect...");
                IconData::default()
            }))
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
        "Soniox Live",
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
        tracing::error!("Soniox Live {:?}", err);
        std::process::exit(1);
    }
}
