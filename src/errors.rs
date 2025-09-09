use wasapi::WasapiError;

#[derive(thiserror::Error, Debug)]
pub enum SonioxWindowsErrors {
    #[error("{0}")]
    Wasapi(#[from] WasapiError),
    #[error("{0}")]
    Url(#[from] url::ParseError),
    #[error("{0}")]
    Websocket(#[from] tungstenite::Error),
    #[error("{0}")]
    Json(#[from] serde_json::Error),
}
