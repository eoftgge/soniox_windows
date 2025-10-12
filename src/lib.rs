use log4rs::append::file::FileAppender;
use log4rs::Config;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use screen_size::get_primary_screen_size;
use tokio::sync::mpsc::unbounded_channel;
use crate::app::SubtitlesApp;
use crate::errors::SonioxWindowsErrors;
use crate::soniox::stream::start_soniox_stream;
use crate::types::audio::AudioMessage;
use crate::types::settings::SettingsApp;
use crate::windows::audio::start_capture_audio;

pub mod app;
pub mod errors;
pub mod soniox;
pub mod types;
pub mod windows;

const FILE_LOG: &str = "soniox.log";

pub fn initialize_app(settings: SettingsApp) -> Result<SubtitlesApp, SonioxWindowsErrors> {
    let level = settings.level()?;
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}\n")))
        .build(FILE_LOG)?;
    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(level))?;
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

    Ok(app)
}