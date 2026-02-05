use tokio::sync::mpsc::{channel, Sender, Receiver};
use crate::errors::SonioxWindowsErrors;
use crate::settings::SettingsApp;
use crate::soniox::client::SonioxClient;
use crate::soniox::request::create_request;
use crate::transcription::audio::AudioSession;
use crate::types::audio::AudioSample;
use crate::types::soniox::SonioxTranscriptionResponse;

pub struct TranscriptionService {
    pub(crate) audio: AudioSession,
    pub transcription: Receiver<SonioxTranscriptionResponse>,
    handle: tokio::task::JoinHandle<()>,
}

impl TranscriptionService {
    pub fn start(settings_app: &SettingsApp) -> Result<Self, SonioxWindowsErrors> {
        let (tx_audio, rx_audio) = channel::<AudioSample>(256);
        let (tx_transcription, rx_transcription) = channel::<SonioxTranscriptionResponse>(256);
        let (tx_recycle, rx_recycle) = channel::<AudioSample>(256);

        let audio = AudioSession::open(tx_audio, rx_recycle)?;
        let request = create_request(settings_app, audio.config())?;
        let mut ws = SonioxClient::new(tx_transcription, tx_recycle, rx_audio);
        audio.play()?;
        
        let handle = tokio::spawn(async move {
            if let Err(e) = ws.start(&request).await {
                tracing::error!("WebSocket error: {:?}", e);
            }
        });

        Ok(Self {
            audio,
            handle,
            transcription: rx_transcription,
        })
    }
}

impl Drop for TranscriptionService {
    fn drop(&mut self) {
        self.handle.abort();
    }
}