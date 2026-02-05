use crate::errors::SonioxWindowsErrors;
use crate::gui::app::SubtitlesApp;
use crate::soniox::stream::start_soniox_stream;
use crate::types::audio::{AudioMessage, AudioSample};
use crate::types::soniox::SonioxTranscriptionResponse;
use crate::windows::audio::start_capture_audio;
use settings::SettingsApp;
use tokio::sync::mpsc::channel;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub mod errors;
pub mod gui;
pub mod settings;
pub mod soniox;
pub mod types;
pub mod windows;

pub const ICON_BYTES: &[u8] = include_bytes!("../assets/icon.png");

fn setup_logging(level: LevelFilter) -> tracing_appender::non_blocking::WorkerGuard {
    let file_appender = tracing_appender::rolling::daily("logs", "soniox.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(std::io::stdout)
                .with_ansi(true),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .with_target(false),
        )
        .with(level)
        .init();

    guard
}

pub fn initialize_app(settings: SettingsApp) -> Result<SubtitlesApp, SonioxWindowsErrors> {
    let level = settings.level()?;
    let _ = setup_logging(level);

    let (tx_audio, rx_audio) = channel::<AudioMessage>(256);
    let (tx_transcription, rx_transcription) = channel::<SonioxTranscriptionResponse>(256);
    let (tx_exit, rx_exit) = channel::<bool>(1);
    let (tx_recycle, rx_recycle) = channel::<AudioSample>(256);
    let app = SubtitlesApp::new(
        rx_transcription,
        tx_exit,
        tx_audio.clone(),
        settings.clone(),
    );
    std::thread::spawn(move || {
        if let Err(err) = start_capture_audio(tx_audio, rx_exit, rx_recycle) {
            tracing::error!("{}", err);
        }
    });
    tokio::spawn(async move {
        if let Err(err) =
            start_soniox_stream(&settings, tx_transcription, rx_audio, tx_recycle).await
        {
            tracing::error!("{}", err);
        }
    });

    Ok(app)
}
