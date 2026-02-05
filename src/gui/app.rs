use crate::gui::draw::draw_subtitles;
use crate::gui::settings::show_settings_window;
use crate::gui::state::AppState;
use crate::settings::SettingsApp;
use crate::transcription::store::TranscriptionStore;
use eframe::egui::{Align, Area, Context, Id, Layout, Order, Visuals};
use eframe::{App, Frame};
use egui_notify::Toasts;
use std::time::Duration;
use crate::transcription::service::TranscriptionService;

const MAX_FPS: u64 = 30;
const FRAME_TIME: Duration = Duration::from_millis(1000 / MAX_FPS);

pub struct SubtitlesApp {
    settings: SettingsApp,
    state: AppState,
    last_state: Option<AppState>,
    store: TranscriptionStore,
    toasts: Toasts,
    service: Option<TranscriptionService>,
}

impl SubtitlesApp {
    pub fn new(
        settings: SettingsApp,
    ) -> Self {
        Self {
            store: TranscriptionStore::new(settings.max_blocks()),
            toasts: Toasts::new(),
            state: AppState::Config,
            last_state: None,
            service: None,
            settings,
        }
    }
}

impl App for SubtitlesApp {
    fn update(&mut self, ctx: &Context, _: &mut Frame) {
        if Some(self.state) != self.last_state {
            self.state
                .apply_window_state(ctx, self.settings.enable_high_priority());

            if self.state == AppState::Overlay {
                self.store.resize(self.settings.max_blocks);
                match TranscriptionService::start(&self.settings) {
                    Ok(service) => self.service = Some(service),
                    Err(err) => tracing::error!("Failed to start service: {:?}", err),
                }
            }

            self.last_state = Some(self.state);
        }

        match self.state {
            AppState::Config => {
                show_settings_window(ctx, &mut self.settings, &mut self.state, &mut self.toasts)
            }
            AppState::Overlay => {
                self.store
                    .clear_if_silent(Duration::from_secs(15));

                if let Some(service) = &mut self.service {
                    while let Ok(response) = service.transcription.try_recv() {
                        self.store.update(response);
                    }
                }

                Area::new(Id::from("subtitles_area"))
                    .fixed_pos(self.settings.get_position())
                    .order(Order::Foreground)
                    .show(ctx, |ui| {
                        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                            draw_subtitles(
                                ui,
                                &self.store,
                                self.settings.font_size,
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

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.service = None;
    }

    fn clear_color(&self, _visuals: &Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0]
    }
}
