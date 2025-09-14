// #![windows_subsystem = "windows"]

use std::str::FromStr;
use eframe::egui::ViewportBuilder;
use eframe::icon_data::from_png_bytes;
use log4rs::append::console::{ConsoleAppender, Target};
use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use log::LevelFilter;
use soniox_windows::app::SubtitlesApp;
use soniox_windows::windows::audio::start_capture_audio;
use soniox_windows::errors::SonioxWindowsErrors;
use soniox_windows::soniox::stream::start_soniox_stream;
use soniox_windows::types::audio::AudioMessage;
use tokio::sync::mpsc::unbounded_channel;
use soniox_windows::types::settings::SettingsApp;

#[tokio::main]
async fn main() -> Result<(), SonioxWindowsErrors> {
    let settings = SettingsApp::new("soniox.toml")?;
    let level = log::LevelFilter::from_str(&settings.level).map_err(|_| SonioxWindowsErrors::Internal("field `level` isn't valid. did u mean `info`, `debug` and `warn`?"))?;
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}\n")))
        .build("soniox.log")?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("logfile")
                .appender("stderr")
                .build(LevelFilter::Trace),
        )?;
    let _ = log4rs::init_config(config);
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
