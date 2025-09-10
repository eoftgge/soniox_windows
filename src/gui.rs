use crate::windows::initialize_windows;
use crossbeam_channel::Receiver;
use eframe::epaint::Color32;
use eframe::{App, Frame, egui};
use egui::Visuals;
use std::time::Duration;

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
            text: "... Wait ...".into(),
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
                let rect = ui.max_rect();
                let text = &self.text;
                let x_offset = 10.0;
                let y_offset = rect.height() - 50.0;
                let pos = egui::pos2(x_offset, y_offset);
                let max_width = ui.available_width() - 20.0;
                let mut font_size = 24.0;

                let galley = ui.painter().layout_no_wrap(
                    text.clone(),
                    egui::FontId::proportional(font_size),
                    Color32::WHITE,
                );

                if galley.size().x > max_width {
                    font_size = font_size * max_width / galley.size().x;
                }
                let offsets = [
                    egui::vec2(-1.0, -1.0),
                    egui::vec2(-1.0, 1.0),
                    egui::vec2(1.0, -1.0),
                    egui::vec2(1.0, 1.0),
                ];

                for offset in &offsets {
                    ui.painter().text(
                        pos + *offset,
                        egui::Align2::LEFT_TOP,
                        text,
                        egui::FontId::proportional(font_size),
                        Color32::BLACK,
                    );
                }

                ui.painter().text(
                    pos,
                    egui::Align2::LEFT_TOP,
                    text,
                    egui::FontId::proportional(font_size),
                    Color32::YELLOW,
                );
            });

        ctx.request_repaint_after(Duration::from_millis(100));
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
