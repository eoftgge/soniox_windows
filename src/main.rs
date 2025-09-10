use std::error::Error;
use std::thread;

use crossbeam_channel::unbounded;
use egui::ViewportBuilder;
use soniox_windows::gui::SubtitlesApp;
use soniox_windows::soniox::start_soniox_stream;
use soniox_windows::stream::start_capture_audio;
use soniox_windows::types::AudioSample;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("SONIOX_APIKEY")?;
    let (tx, rx) = unbounded::<AudioSample>();
    let (tx_text, rx_text) = unbounded::<String>();
    let app = SubtitlesApp::new(rx_text);
    thread::spawn(move || start_capture_audio(tx));
    tokio::spawn(start_soniox_stream(rx, api_key, tx_text));

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

    let _ = eframe::run_native("Subtitle Live", native_options, Box::new(move |_| {
        Ok(Box::new(app))
    }));
    Ok(())
}
