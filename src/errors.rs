use log4rs::config::runtime::ConfigErrors;
use wasapi::WasapiError;

#[derive(thiserror::Error, Debug)]
pub enum SonioxWindowsErrors {
    #[error("Error in audio: {0}")]
    Wasapi(#[from] WasapiError),
    #[error("Error in Windows API: {0}")]
    Windows(#[from] windows::core::Error),
    #[error("Error in WEB: {0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("Error in JSON: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Error in Graphics: {0}")]
    Graphics(#[from] eframe::Error),
    #[error("Error in config: {0}")]
    Config(#[from] config::ConfigError),
    #[error("Error in IO: {0}")]
    Io(#[from] std::io::Error),
    #[error("Error in log: {0}")]
    LoggingRuntime(#[from] ConfigErrors),
    #[error("Error in string UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Internal error: {0}")]
    Internal(&'static str),
    #[error("Error in WebSocket... Maybe he is dead")]
    WssConnectionError,
}
