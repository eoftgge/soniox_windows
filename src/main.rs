#![windows_subsystem = "windows"]

use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use log::LevelFilter;
use log4rs::Config;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use screen_size::get_primary_screen_size;
use soniox_windows::app::SubtitlesApp;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::soniox::stream::start_soniox_stream;
use soniox_windows::types::audio::AudioMessage;
use soniox_windows::types::settings::SettingsApp;
use soniox_windows::windows::audio::start_capture_audio;
use std::str::FromStr;
use tokio::sync::mpsc::unbounded_channel;

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
    let level = LevelFilter::from_str(&settings.level).map_err(|_| {
        SonioxWindowsErrors::Internal(
            "field `level` isn't valid. did u mean `info`, `debug` and `warn`?",
        )
    })?;
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}\n")))
        .build("soniox.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(level))?;
    let _ = log4rs::init_config(config);
    let (width, height) = get_primary_screen_size().expect("Failed to get primary screen size");
    let (tx_audio, rx_audio) = unbounded_channel::<AudioMessage>();
    let (tx_subs, rx_subs) = unbounded_channel::<String>();
    let app = SubtitlesApp::new(rx_subs, tx_audio.clone());
    tokio::task::spawn_blocking(move || {
        if let Err(err) = start_capture_audio(tx_audio) {
            log::error!("{}", err);
        }
    });
    tokio::spawn(async move {
        if let Err(err) = start_soniox_stream(settings, tx_subs, rx_audio).await {
            log::error!("{}", err);
        }
    });
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
