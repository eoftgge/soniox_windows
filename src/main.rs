#![windows_subsystem = "windows"]
use std::thread;

use crossbeam_channel::unbounded;
use eframe::egui::ViewportBuilder;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::gui::SubtitlesApp;
use soniox_windows::soniox::start_soniox_stream;
use soniox_windows::audio::start_capture_audio;
use soniox_windows::types::AudioSample;

#[tokio::main]
async fn main() -> Result<(), SonioxWindowsErrors> {
    simple_logger::SimpleLogger::new().init().unwrap();

    let api_key = std::env::var("SONIOX_APIKEY")?;
    let (tx, rx) = unbounded::<AudioSample>();
    let (tx_text, rx_text) = unbounded::<String>();
    let app = SubtitlesApp::new(rx_text);
    thread::spawn(move || {
        if let Err(err) = start_capture_audio(tx) {
            log::error!("{}", err);
        }
    });
    tokio::spawn(async move {
        if let Err(err) = start_soniox_stream(rx, api_key, tx_text).await {
            log::error!("{}", err);
        }
    });

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true)
            .with_min_inner_size([800., 100.])
            .with_inner_size([800., 100.])
            .with_max_inner_size([800., 100.])
            .with_position([100., (1920 - 1200) as f32]),
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
