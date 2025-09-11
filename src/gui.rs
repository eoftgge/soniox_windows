use crate::windows::initialize_windows;
use crossbeam_channel::Receiver;
use eframe::epaint::Color32;
use eframe::{App, Frame, egui};
use egui::Visuals;
use std::time::Duration;

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
    let pos = rect.left_bottom() + egui::vec2(10., -40.);
    let font = egui::FontId::proportional(font_size);
    let offsets = [
        egui::vec2(-thickness, 0.0),
        egui::vec2(thickness, 0.0),
        egui::vec2(0.0, -thickness),
        egui::vec2(0.0, thickness),
        egui::vec2(-thickness, -thickness),
        egui::vec2(-thickness, thickness),
        egui::vec2(thickness, -thickness),
        egui::vec2(thickness, thickness),
    ];

    for offset in offsets {
        painter.text(
            pos + offset,
            egui::Align2::LEFT_BOTTOM,
            &modified,
            font.clone(),
            outline_color,
        );
    }
    painter.text(
        pos,
        egui::Align2::LEFT_BOTTOM,
        &modified,
        egui::FontId::proportional(font_size),
        text_color,
    );
}

pub struct SubtitlesApp {
    rx: Receiver<String>,
    text: String,
    initialized_windows: bool,
}

impl SubtitlesApp {
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            rx,
            initialized_windows: false,
            text: "... waiting for the sound ...".into(),
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        if !self.initialized_windows {
            initialize_windows(frame);
        }
        while let Ok(new_text) = self.rx.try_recv() {
            self.text = new_text;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    draw_text_with_shadow(ui, &self.text, 24.0);
                });
            });

        ctx.request_repaint_after(Duration::from_millis(100));
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
