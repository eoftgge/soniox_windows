use crate::errors::SonioxLiveErrors;
use crate::settings::SettingsApp;
use crate::soniox::client::SonioxClient;
use crate::soniox::request::create_request;
use crate::transcription::audio::AudioSession;
use crate::types::audio::AudioSample;
use crate::types::events::SonioxEvent;
use tokio::sync::mpsc::{Receiver, channel};

pub struct TranscriptionService {
    pub(crate) _audio: AudioSession,
    pub receiver: Receiver<SonioxEvent>,
    handle: tokio::task::JoinHandle<()>,
}

impl TranscriptionService {
    pub fn start(settings_app: &SettingsApp) -> Result<Self, SonioxLiveErrors> {
        let (tx_event, rx_event) = channel::<SonioxEvent>(128);
        let (tx_audio, rx_audio) = channel::<AudioSample>(256);
        let (tx_recycle, rx_recycle) = channel::<AudioSample>(256);

        let audio = AudioSession::open(tx_audio, rx_recycle)?;
        let request = create_request(settings_app, audio.config())?;
        let mut ws = SonioxClient::new(tx_event, tx_recycle, rx_audio);
        audio.play()?;

        let handle = tokio::spawn(async move {
            if let Err(e) = ws.start(&request).await {
                tracing::error!("WebSocket error: {:?}", e);
            }
        });

        Ok(Self {
            _audio: audio,
            handle,
            receiver: rx_event,
        })
    }
}

impl Drop for TranscriptionService {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
