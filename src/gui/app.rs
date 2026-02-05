use crate::gui::draw::draw_subtitles;
use crate::gui::settings::show_settings_window;
use crate::gui::state::AppState;
use crate::settings::SettingsApp;
use crate::soniox::transcription::TranscriptionStore;
use crate::types::audio::AudioMessage;
use crate::types::soniox::SonioxTranscriptionResponse;
use eframe::egui::{Align, Area, Context, Id, Layout, Order, Visuals};
use eframe::{App, Frame};
use egui_notify::Toasts;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};

const MAX_FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    rx_transcription: Receiver<SonioxTranscriptionResponse>,
    tx_audio: Sender<AudioMessage>,
    tx_exit: Sender<bool>,
    settings: SettingsApp,
    state: AppState,
    last_state: Option<AppState>,
    transcription_store: TranscriptionStore,
    toasts: Toasts,
}

impl SubtitlesApp {
    pub fn new(
        rx_transcription: Receiver<SonioxTranscriptionResponse>,
        tx_exit: Sender<bool>,
        tx_audio: Sender<AudioMessage>,
        settings: SettingsApp,
    ) -> Self {
        Self {
            transcription_store: TranscriptionStore::new(settings.max_blocks()),
            rx_transcription,
            tx_exit,
            tx_audio,
            settings,
            toasts: Toasts::new(),
            state: AppState::Config,
            last_state: None,
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if Some(self.state) != self.last_state {
            self.state
                .apply_window_state(ctx, self.settings.enable_high_priority());

            if self.state == AppState::Overlay {
                self.transcription_store.resize(self.settings.max_blocks);
            }

            self.last_state = Some(self.state);
        }

        match self.state {
            AppState::Config => {
                show_settings_window(ctx, &mut self.settings, &mut self.state, &mut self.toasts)
            }
            AppState::Overlay => {
                self.transcription_store
                    .clear_if_silent(Duration::from_secs(15));
                while let Ok(transcription) = self.rx_transcription.try_recv() {
                    self.transcription_store.update(transcription);
                }

                Area::new(Id::from("subtitles_area"))
                    .fixed_pos(self.settings.get_position())
                    .order(Order::Foreground)
                    .show(ctx, |ui| {
                        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                            draw_subtitles(
                                ui,
                                &self.transcription_store,
                                self.settings.font_size,
                                self.settings.text_color(),
                                self.settings.get_background_color(),
                            );
                        });
                        ctx.request_repaint_after(FRAME_TIME);
                    });
            }
        }
        self.toasts.show(ctx);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.tx_audio.send(AudioMessage::Stop);
        let _ = self.tx_exit.send(true);
        self.rx_transcription.close();
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
