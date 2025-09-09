use wasapi::WasapiError;

#[derive(thiserror::Error, Debug)]
pub enum SonioxWindowsErrors {
    #[error("{0}")]
    Wasapi(#[from] WasapiError),
}
