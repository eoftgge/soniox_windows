use crate::errors::SonioxLiveErrors;
use crate::settings::SettingsApp;
use crate::soniox::request::create_request;
use crate::soniox::worker::SonioxWorker;
use crate::transcription::audio::AudioSession;
use crate::types::audio::AudioSample;
use crate::types::events::SonioxEvent;
use eframe::egui::Context;
use tokio::sync::mpsc::{Receiver, channel};

pub struct TranscriptionService {
    pub(crate) _audio: AudioSession,
    pub receiver: Receiver<SonioxEvent>,
    handle: tokio::task::JoinHandle<()>,
}

impl TranscriptionService {
    pub fn start(ctx: Context, settings_app: &SettingsApp) -> Result<Self, SonioxLiveErrors> {
        let (tx_worker, mut rx_worker) = channel::<SonioxEvent>(128);
        let (tx_event, rx_event) = channel::<SonioxEvent>(128);
        let (tx_audio, rx_audio) = channel::<AudioSample>(256);
        let (tx_recycle, rx_recycle) = channel::<AudioSample>(256);

        let tx_worker_2 = tx_worker.clone();
        let worker = SonioxWorker::new(rx_audio, tx_recycle, tx_worker_2);
        let audio = AudioSession::open(tx_audio, rx_recycle)?;
        let request = create_request(settings_app, audio.config())?;
        audio.play()?;

        let handle = tokio::spawn(async move {
            if let Err(e) = worker.run(&request).await {
                tracing::error!("WebSocket error: {:?}", e);
                let _ = tx_worker.send(SonioxEvent::Error(e)).await;
            }
        });
        tokio::spawn(async move {
            while let Some(event) = rx_worker.recv().await {
                if tx_event.send(event).await.is_err() {
                    break;
                }
                ctx.request_repaint();
            }
        });

        Ok(Self {
            _audio: audio,
            handle,
            receiver: rx_event,
        })
    }

    pub fn listen() {}
}

impl Drop for TranscriptionService {
    fn drop(&mut self) {
        self.handle.abort();
    }
}
