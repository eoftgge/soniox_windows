use crate::gui::draw::draw_subtitles;
use crate::soniox::transcription::TranscriptionStore;
use crate::types::audio::AudioMessage;
use crate::settings::SettingsApp;
use crate::types::soniox::SonioxTranscriptionResponse;
use crate::windows::utils::{initialize_tool_window, initialize_window, make_window_click_through};
use eframe::egui::{Align, Area, Context, Id, Layout, Order, Pos2, Visuals};
use eframe::epaint::Color32;
use eframe::{App, Frame};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const MAX_FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    rx_transcription: UnboundedReceiver<SonioxTranscriptionResponse>,
    tx_audio: UnboundedSender<AudioMessage>,
    tx_exit: UnboundedSender<bool>,
    enable_high_priority: bool,
    font_size: f32,
    text_color: Color32,
    background_color: Color32,
    position: Pos2,
    initialized_windows: bool,
    transcription_store: TranscriptionStore,
}

impl SubtitlesApp {
    pub fn new(
        rx_transcription: UnboundedReceiver<SonioxTranscriptionResponse>,
        tx_exit: UnboundedSender<bool>,
        tx_audio: UnboundedSender<AudioMessage>,
        settings: &SettingsApp, // maybe add builder?
    ) -> Self {
        Self {
            rx_transcription,
            tx_exit,
            tx_audio,
            enable_high_priority: settings.enable_high_priority(),
            font_size: settings.font_size(),
            text_color: settings.text_color(),
            background_color: settings.get_background_color(),
            position: settings.get_position(),
            initialized_windows: false,
            transcription_store: TranscriptionStore::new(settings.max_blocks()),
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        Area::new(Id::from("subtitles_area"))
            .fixed_pos(self.position)
            .order(Order::Foreground)
            .show(ctx, |ui| {
                make_window_click_through(frame);
                if !self.initialized_windows {
                    initialize_window(frame);
                    self.initialized_windows = true;
                }
                if self.enable_high_priority {
                    initialize_tool_window(frame);
                }

                while let Ok(transcription) = self.rx_transcription.try_recv() {
                    self.transcription_store.update(transcription);
                }

                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    draw_subtitles(
                        ui,
                        &self.transcription_store,
                        self.font_size,
                        self.text_color,
                        self.background_color,
                    );
                });

                ctx.request_repaint_after(FRAME_TIME);
            });
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
