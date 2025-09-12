use crate::settings::SettingsApp;
use crate::types::AudioMessage;
use crate::utils_windows::initialize_windows;
use eframe::epaint::Color32;
use eframe::glow::Context;
use eframe::{egui, App, Frame};
use egui::{vec2, Align2, FontId, Visuals};
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

fn trim_text_to_fit(text: &str, max_chars: usize) -> String {
    if text.chars().count() > max_chars {
        let tail: String = text.chars().rev().take(max_chars).collect();
        tail.chars().rev().collect()
    } else {
        text.to_string()
    }
}

#[inline]
fn modify_text(text: &str) -> String {
    text.replace("--", "—")
}

fn draw_text_with_shadow(ui: &mut egui::Ui, text: &str, font_size: f32) {
    let trimmed = trim_text_to_fit(text, 50);
    let modified = modify_text(&trimmed);
    let outline_color = Color32::BLACK;
    let text_color = Color32::YELLOW;
    let thickness = 2.0;

    let painter = ui.painter();
    let rect = ui.ctx().screen_rect();
    let pos = rect.left_bottom() + vec2(10., -40.);
    let font = FontId::proportional(font_size);
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
            &modified,
            font.clone(),
            outline_color,
        );
    }
    painter.text(
        pos,
        Align2::LEFT_BOTTOM,
        &modified,
        FontId::proportional(font_size),
        text_color,
    );
}

pub struct SubtitlesApp {
    tx_audio: UnboundedSender<AudioMessage>,
    rx_subs: Arc<Mutex<UnboundedReceiver<String>>>,
    text: Arc<Mutex<String>>,
    settings: Arc<Mutex<SettingsApp>>,
}

impl SubtitlesApp {
    pub fn new(
        rx_subs: UnboundedReceiver<String>,
        tx_audio: UnboundedSender<AudioMessage>,
    ) -> Self {
        Self {
            tx_audio,
            rx_subs: Arc::new(Mutex::new(rx_subs)),
            settings: Arc::new(Mutex::new(SettingsApp {
                language_hints: vec![],
                context: String::new(),
            })),
            text: Arc::new(Mutex::new("... waiting for the sound ...".into())),
        }
    }

    fn update_text(&mut self) {
        let text = Arc::clone(&self.text);
        let rx_subs = Arc::clone(&self.rx_subs);
        tokio::task::spawn_blocking(move || {
            let mut rx_subs = rx_subs.lock().unwrap();
            while let Ok(new_text) = rx_subs.try_recv() {
                *text.lock().unwrap() = new_text;
            }
        });
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                initialize_windows(frame);
                self.update_text();
                let text = self.text.lock().unwrap().clone();
                ui.vertical(|ui| {
                    draw_text_with_shadow(ui, &text, 24.0);
                });
            });
        ctx.request_repaint_after(Duration::from_millis(10));
    }

    fn on_exit(&mut self, _gl: Option<&Context>) {
        let _ = self.tx_audio.send(AudioMessage::Stop);
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
