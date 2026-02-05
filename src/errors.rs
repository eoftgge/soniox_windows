#[derive(thiserror::Error, Debug)]
pub enum SonioxLiveErrors {
    #[error("Error in builder stream")]
    BuildStream(#[from] cpal::BuildStreamError),
    #[error("Error in player stream")]
    PlayStream(#[from] cpal::PlayStreamError),
    #[error("Error in stream config")]
    DefaultStreamConfig(#[from] cpal::DefaultStreamConfigError),
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
    #[error("Error in string UTF-8: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Internal error: {0}")]
    Internal(&'static str),
    #[error("Internal error: {0}")]
    OwnedInternal(String),
    #[error("Error in WebSocket... Maybe he is dead")]
    WssConnectionError,
}
