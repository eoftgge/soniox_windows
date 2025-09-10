use crossbeam_channel::Receiver;
use eframe::{App, Frame, egui};
use std::time::Duration;
use eframe::epaint::Color32;
use egui::Visuals;

pub struct SubtitlesApp {
    rx: Receiver<String>,
    text: String,
}

impl SubtitlesApp {
    pub fn new(rx: Receiver<String>) -> Self {
        Self {
            rx,
            text: "... Wait ...".into(),
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut Frame) {
        while let Ok(new_text) = self.rx.try_recv() {
            self.text = new_text;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(Color32::TRANSPARENT))
            .show(ctx, |ui| {
            ui.label(egui::RichText::new(&self.text).color(Color32::YELLOW).size(24.0));
        });

        ctx.request_repaint_after(Duration::from_millis(100));
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
