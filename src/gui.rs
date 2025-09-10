use crossbeam_channel::Receiver;
use eframe::epaint::Color32;
use eframe::{egui, App, Frame};
use egui::Visuals;
use std::time::Duration;
use raw_window_handle::HasWindowHandle;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_TRANSPARENT};

fn make_window_click_through(hwnd: HWND) {
    unsafe {
        let ex_style = GetWindowLongW(hwnd, GWL_EXSTYLE);
        SetWindowLongW(hwnd, GWL_EXSTYLE, ex_style | WS_EX_LAYERED.0 as i32 | WS_EX_TRANSPARENT.0 as i32);
    }
}

fn set_click_through(frame: &eframe::Frame) {
    use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};

    match frame.window_handle() {
        Ok(handle) => {
            let raw = handle.as_raw();
            if let RawWindowHandle::Win32(win32) = raw {
                let hwnd = HWND(win32.hwnd.get() as *mut _);
                make_window_click_through(hwnd);
            }
        }
        _ => {}
    }
}

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
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        set_click_through(frame);
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
