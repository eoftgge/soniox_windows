use eframe::{App, Frame, epaint::Color32};
use eframe::glow::Context;
use egui::Visuals;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::gui::draw::draw_text_with_shadow;
use crate::types::audio::{AudioMessage, AudioSubtitle};
use crate::windows::utils::{initialize_windows, make_window_click_through};

pub mod text;
pub mod draw;

pub struct SubtitlesApp {
    tx_audio: UnboundedSender<AudioMessage>,
    rx_subs: UnboundedReceiver<AudioSubtitle>,
    subtitle: AudioSubtitle,
    initialized_windows: bool,
}

impl SubtitlesApp {
    pub fn new(
        rx_subs: UnboundedReceiver<AudioSubtitle>,
        tx_audio: UnboundedSender<AudioMessage>,
    ) -> Self {
        Self {
            tx_audio,
            rx_subs,
            initialized_windows: false,
            subtitle: AudioSubtitle::default(),
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                make_window_click_through(frame);
                if !self.initialized_windows {
                    initialize_windows(frame);
                    self.initialized_windows = true;
                }
                while let Ok(subtitle) = self.rx_subs.try_recv() {
                    self.subtitle = subtitle;
                }
                ui.vertical(|ui| {
                    draw_text_with_shadow(ui, &self.subtitle, 24.0);
                });
                ctx.request_repaint();
            });
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        let _ = self.tx_audio.send(AudioMessage::Stop);
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
