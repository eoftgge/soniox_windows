use crate::gui::draw::draw_subtitles;
use crate::gui::settings::show_settings_window;
use crate::gui::state::{AppState, StateManager};
use crate::settings::SettingsApp;
use crate::transcription::store::TranscriptionStore;
use eframe::egui::{Align, Area, Context, Id, Layout, Order, Visuals};
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
    _guard: WorkerGuard,
}

impl SubtitlesApp {
    pub fn new(settings: SettingsApp, guard: WorkerGuard) -> Self {
        Self {
            store: TranscriptionStore::new(settings.max_blocks()),
            toasts: Toasts::new(),
            manager: StateManager::new(),
            settings,
            _guard: guard,
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if let Err(err) = self.manager.resolve(ctx, &mut self.store, &self.settings) {
            self.toasts.error(format!("{:?}", err));
        }

        match self.manager.app_state_mut() {
            AppState::Config => {
                show_settings_window(ctx, &mut self.settings, &mut self.manager, &mut self.toasts)
            }
            AppState::Overlay(service) => {
                self.store.clear_if_silent(Duration::from_secs(15));

                while let Ok(response) = service.transcription.try_recv() {
                    self.store.update(response);
                }

                Area::new(Id::from("subtitles_area"))
                    .fixed_pos(self.settings.get_position())
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
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
