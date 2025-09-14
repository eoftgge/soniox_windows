use log4rs::config::runtime::ConfigErrors;
use std::env;
use wasapi::WasapiError;

#[derive(thiserror::Error, Debug)]
pub enum SonioxWindowsErrors {
    #[error("{0}")]
    Wasapi(#[from] WasapiError),
    #[error("{0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[error("{0}")]
    Egui(#[from] eframe::Error),
    #[error("{0}")]
    Env(#[from] env::VarError),
    #[error("{0}")]
    Config(#[from] config::ConfigError),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    LoggingRuntime(#[from] ConfigErrors),
    #[error("{0}")]
    Internal(&'static str),
}
