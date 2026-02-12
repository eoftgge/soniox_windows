use crate::gui::draw::draw_subtitles;
use crate::gui::settings::show_settings_window;
use crate::gui::state::{AppState, StateManager};
use crate::settings::SettingsApp;
use crate::transcription::store::TranscriptionStore;
use crate::types::events::SonioxEvent;
use eframe::egui::{
    Align, Area, Context, Id, Layout, Order, ViewportCommand, Visuals, WindowLevel,
};
use eframe::{App, Frame};
use egui_notify::Toasts;
use std::time::Duration;
use tracing_appender::non_blocking::WorkerGuard;

const MAX_FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    settings: SettingsApp,
    store: TranscriptionStore,
    toasts: Toasts,
    manager: StateManager,
    frame_counter: u64,
    _guard: WorkerGuard,
}

impl SubtitlesApp {
    pub fn new(settings: SettingsApp, guard: WorkerGuard) -> Self {
        Self {
            store: TranscriptionStore::new(settings.max_blocks()),
            toasts: Toasts::new(),
            manager: StateManager::new(),
            settings,
            frame_counter: 0,
            _guard: guard,
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if let Err(err) = self.manager.resolve(ctx, &mut self.store, &self.settings) {
            self.toasts.error(format!("{:?}", err));
        }
        let manager = &mut self.manager;

        match manager.app_state_mut() {
            AppState::Config => {
                show_settings_window(ctx, &mut self.settings, &mut self.manager, &mut self.toasts)
            }
            AppState::Overlay(service) => {
                self.store.clear_if_silent(Duration::from_secs(15));

                while let Ok(event) = service.receiver.try_recv() {
                    match event {
                        SonioxEvent::Transcription(r) => self.store.update(r),
                        SonioxEvent::Warning(s) => {
                            self.toasts
                                .warning(s.to_string())
                                .duration(Duration::from_secs(4));
                        }
                        SonioxEvent::Error(e) => {
                            self.toasts
                                .error(e.to_string())
                                .duration(Duration::from_secs(4));
                        }
                    };
                }

                if self.settings.enable_high_priority() && self.frame_counter >= 100 {
                    ctx.send_viewport_cmd(ViewportCommand::WindowLevel(WindowLevel::AlwaysOnTop));
                    self.frame_counter = 0;
                }

                let (anchor, offset) = self.settings.get_anchor();
                Area::new(Id::from("subtitles_area"))
                    .anchor(anchor, offset)
                    .order(Order::Foreground)
                    .show(ctx, |ui| {
                        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                            draw_subtitles(
                                ui,
                                &self.store,
                                self.settings.font_size(),
                                self.settings.text_color(),
                                self.settings.get_background_color(),
                            );
                        });
                        ctx.request_repaint_after(FRAME_TIME);
                    });
            }
        }

        self.toasts.show(ctx);
        self.frame_counter += 1;
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
