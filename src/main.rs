// #![windows_subsystem = "windows"]

use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use log::LevelFilter;
use soniox_windows::app::SubtitlesApp;
use soniox_windows::audio::start_capture_audio;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::soniox::start_soniox_stream;
use soniox_windows::types::AudioMessage;
use tokio::sync::mpsc::unbounded_channel;

#[tokio::main]
async fn main() -> Result<(), SonioxWindowsErrors> {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let api_key = std::env::var("SONIOX_APIKEY")?;
    let (tx_audio, rx_audio) = unbounded_channel::<AudioMessage>();
    let (tx_subs, rx_subs) = unbounded_channel::<String>();
    let app = SubtitlesApp::new(rx_subs, tx_audio.clone());
    tokio::task::spawn_blocking(move || {
        if let Err(err) = start_capture_audio(tx_audio) {
            log::error!("{}", err);
        }
    });
    tokio::spawn(async move {
        if let Err(err) = start_soniox_stream(api_key, tx_subs, rx_audio).await {
            log::error!("{}", err);
        }
    });
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_app_id("sublive")
            .with_icon(from_png_bytes(include_bytes!("../icon.png")).expect("Failed to load icon"))
            .with_decorations(false)
            .with_always_on_top()
            .with_transparent(true)
            .with_min_inner_size([1000., 250.])
            .with_inner_size([1000., 250.])
            .with_max_inner_size([1000., 250.])
            .with_position([100., (1920 - 1300) as f32]),
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
