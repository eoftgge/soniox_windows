use crate::types::audio::{AudioMessage, AudioSubtitle};
use crate::windows::utils::{initialize_windows, make_window_click_through};
use eframe::epaint::Color32;
use eframe::glow::Context;
use eframe::{App, Frame, egui};
use egui::{Align2, FontId, Visuals, vec2};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

fn trim_text_to_fit_precise(
    text: String,
    ui: &egui::Ui,
    font_id: &FontId,
    max_width_ratio: f32,
) -> String {
    let available_width = ui.ctx().content_rect().width() * max_width_ratio;
    let mut chars: Vec<char> = text.chars().collect();
    let mut trimmed = text.to_owned();

    loop {
        let galley = ui
            .painter()
            .layout_no_wrap(trimmed.clone(), font_id.clone(), Color32::WHITE);
        let text_width = galley.size().x;

        if text_width <= available_width || chars.len() <= 4 {
            break;
        }

        chars.remove(0);
        trimmed = format!("...{}", chars.iter().collect::<String>().trim_start());
    }

    trimmed
}

#[inline]
fn modify_text(text: &str) -> String {
    text.replace("--", "—")
}

fn draw_text_with_shadow(ui: &mut egui::Ui, subtitle: &AudioSubtitle, font_size: f32) {
    let text = match subtitle {
        AudioSubtitle::Text(text) => modify_text(text),
        AudioSubtitle::Speaker(speaker, text) => format!("{}: {}", speaker, modify_text(text)),
        AudioSubtitle::Empty => return,
    };

    let font = FontId::proportional(font_size);
    let trimmed = trim_text_to_fit_precise(text, ui, &font, 0.8);
    let outline_color = Color32::BLACK;
    let text_color = Color32::YELLOW;
    let thickness = 2.0;

    let painter = ui.painter();
    let rect = ui.ctx().content_rect();
    let pos = rect.left_bottom() + vec2(10., -40.);
    let offsets = [
        vec2(-thickness, 0.0),
        vec2(thickness, 0.0),
        vec2(0.0, -thickness),
        vec2(0.0, thickness),
        vec2(-thickness, -thickness),
        vec2(-thickness, thickness),
        vec2(thickness, -thickness),
        vec2(thickness, thickness),
    ];

    for offset in offsets {
        painter.text(
            pos + offset,
            Align2::LEFT_BOTTOM,
            &trimmed,
            font.clone(),
            outline_color,
        );
    }
    painter.text(pos, Align2::LEFT_BOTTOM, &trimmed, font, text_color);
}

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
