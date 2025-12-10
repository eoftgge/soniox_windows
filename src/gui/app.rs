use crate::gui::draw::draw_text_with_shadow;
use crate::types::audio::{AudioMessage, AudioSubtitle};
use crate::windows::utils::{initialize_tool_window, initialize_window, make_window_click_through};
use eframe::egui::{CentralPanel, Context, Visuals};
use eframe::epaint::Color32;
use eframe::{App, Frame};
use std::thread::sleep;
use std::time::Duration;
use futures_util::SinkExt;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

const MAX_FPS: u64 = 60;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    rx_subs: UnboundedReceiver<AudioSubtitle>,
    tx_audio: UnboundedSender<AudioMessage>,
    tx_exit: UnboundedSender<bool>,
    subtitle: AudioSubtitle,
    initialized_windows: bool,
    enable_high_priority: bool,
    font_size: f32,
    text_color: Color32,
}

impl SubtitlesApp {
    pub fn new(
        rx_subs: UnboundedReceiver<AudioSubtitle>,
        tx_exit: UnboundedSender<bool>,
        tx_audio: UnboundedSender<AudioMessage>,
        enable_high_priority: bool,
        font_size: f32,
        text_color: Color32,
    ) -> Self {
        Self {
            rx_subs,
            tx_exit,
            tx_audio,
            enable_high_priority,
            font_size,
            text_color,
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
                    draw_text_with_shadow(ui, &self.subtitle, self.font_size, self.text_color);
                });
                ctx.request_repaint_after(FRAME_TIME);
            });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let _ = self.tx_audio.send(AudioMessage::Stop);
        let _= self.tx_exit.send(true);
        self.rx_subs.close();
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
