use thiserror::Error;

#[derive(Error, Debug)]
pub enum SonioxLiveErrors {
    #[error("Audio output stream creation failed")]
    AudioBuildStream(#[from] cpal::BuildStreamError),
    #[error("Audio playback failure")]
    AudioPlayStream(#[from] cpal::PlayStreamError),
    #[error("Failed to get default audio config")]
    AudioConfig(#[from] cpal::DefaultStreamConfigError),
    #[error("WebSocket connection error: {0}")]
    WebSocket(#[from] tungstenite::Error),
    #[error("Server connection lost (Heartbeat failed)")]
    ConnectionLost,
    #[error("Output device is not found")]
    NotFoundOutputDevice,
    #[error("Failed to parse JSON: {0}")]
    JsonParse(#[from] serde_json::Error),
    #[error("Failed to serialize configuration: {0}")]
    ConfigSave(#[from] toml::ser::Error),
    #[error("Failed to load configuration: {0}")]
    ConfigLoad(#[from] toml::de::Error),
    #[error("Invalid UTF-8 sequence: {0}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("Filesystem I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("GUI framework error: {0}")]
    Graphics(#[from] eframe::Error),
    #[error("Internal application error: {0}")]
    Internal(String),
}

impl From<&str> for SonioxLiveErrors {
    fn from(s: &str) -> Self {
        SonioxLiveErrors::Internal(s.to_string())
    }
}
