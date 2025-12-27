use crate::gui::draw::draw_text_with_shadow;
use crate::soniox::state::TranscriptionState;
use crate::types::audio::AudioMessage;
use crate::types::soniox::SonioxTranscriptionResponse;
use crate::windows::utils::{initialize_tool_window, initialize_window, make_window_click_through};
use eframe::egui::{CentralPanel, Context, Visuals};
use eframe::epaint::Color32;
use eframe::{App, Frame};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const MAX_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    rx_transcription: UnboundedReceiver<SonioxTranscriptionResponse>,
    tx_audio: UnboundedSender<AudioMessage>,
    tx_exit: UnboundedSender<bool>,
    initialized_windows: bool,
    enable_high_priority: bool,
    font_size: f32,
    text_color: Color32,
    subtitles_state: TranscriptionState,
}

impl SubtitlesApp {
    pub fn new(
        rx_transcription: UnboundedReceiver<SonioxTranscriptionResponse>,
        tx_exit: UnboundedSender<bool>,
        tx_audio: UnboundedSender<AudioMessage>,
        enable_high_priority: bool,
        font_size: f32,
        text_color: Color32,
    ) -> Self {
        Self {
            rx_transcription,
            tx_exit,
            tx_audio,
            enable_high_priority,
            font_size,
            text_color,
            initialized_windows: false,
            subtitles_state: TranscriptionState::new(3),
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        CentralPanel::default()
            .frame(eframe::egui::Frame::default().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                make_window_click_through(frame);
                if !self.initialized_windows {
                    initialize_window(frame);
                    self.initialized_windows = true;
                }
                if self.enable_high_priority {
                    initialize_tool_window(frame);
                }
                if let Ok(transcription) = self.rx_transcription.try_recv() {
                    self.subtitles_state.handle_transcription(transcription);
                }
                ui.vertical(|ui| {
                    draw_text_with_shadow(
                        ui,
                        self.subtitles_state.get_view_data(),
                        self.font_size,
                        self.text_color,
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
