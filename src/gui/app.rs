use crate::gui::draw::draw_text_with_shadow;
use crate::types::audio::{AudioMessage, AudioSubtitle};
use crate::windows::utils::{initialize_tool_window, initialize_window, make_window_click_through};
use eframe::{App, Frame};
use eframe::epaint::Color32;
use eframe::egui::{Visuals, CentralPanel, Context};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const MAX_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    tx_audio: UnboundedSender<AudioMessage>,
    rx_subs: UnboundedReceiver<AudioSubtitle>,
    subtitle: AudioSubtitle,
    initialized_windows: bool,
    enable_high_priority: bool,
    font_size: f32,
}

impl SubtitlesApp {
    pub fn new(
        rx_subs: UnboundedReceiver<AudioSubtitle>,
        tx_audio: UnboundedSender<AudioMessage>,
        enable_high_priority: bool,
        font_size: f32,
    ) -> Self {
        Self {
            tx_audio,
            rx_subs,
            enable_high_priority,
            font_size,
            initialized_windows: false,
            subtitle: AudioSubtitle::default(),
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
                while let Ok(subtitle) = self.rx_subs.try_recv() {
                    self.subtitle = subtitle;
                }
                ui.vertical(|ui| {
                    draw_text_with_shadow(ui, &self.subtitle, self.font_size);
                });
                ctx.request_repaint();
                sleep(FRAME_TIME);
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.tx_audio.send(AudioMessage::Stop);
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
