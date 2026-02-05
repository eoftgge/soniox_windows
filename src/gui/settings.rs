use crate::gui::state::{PendingState, StateManager};
use crate::settings::SettingsApp;
use crate::types::languages::LanguageHint;
use eframe::egui::{
    self, ComboBox, Context, DragValue, Grid, RichText, ScrollArea, Slider, TextEdit, Ui,
};
use eframe::epaint::Color32;
use egui_notify::Toasts;
use std::time::Duration;

pub fn show_settings_window(
    ctx: &Context,
    settings: &mut SettingsApp,
    manager: &mut StateManager,
    toasts: &mut Toasts,
) {
    ui_bottom_panel(ctx, settings, manager, toasts);

    egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(15.0))
        .show(ctx, |ui| {
            ui.spacing_mut().item_spacing = egui::vec2(8.0, 12.0);
            ui.heading("Settings");
            ui.separator();

            ScrollArea::vertical().show(ui, |ui| {
                ui_log_level(ui, settings);
                ui_section_api(ui, settings);
                ui_section_position(ui, ctx, settings);
                ui_section_appearance(ui, settings);
                ui.allocate_space(egui::vec2(0.0, 60.0));
            });
        });
}

fn ui_bottom_panel(
    ctx: &Context,
    settings: &mut SettingsApp,
    manager: &mut StateManager,
    toasts: &mut Toasts,
) {
    egui::TopBottomPanel::bottom("settings_bottom_panel")
        .resizable(false)
        .min_height(60.0)
        .show(ctx, |ui| {
            ui.add_space(15.0);
            ui.columns(2, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    if ui
                        .add(egui::Button::new("💾 Save").min_size(egui::vec2(0.0, 40.0)))
                        .clicked()
                    {
                        match settings.save("soniox.toml") {
                            Ok(_) => {
                                toasts
                                    .success("Settings saved successfully!")
                                    .duration(Duration::from_secs(3))
                                    .closable(false);
                            }
                            Err(e) => {
                                toasts
                                    .error(format!("Failed to save: {}", e))
                                    .duration(Duration::from_secs(5))
                                    .closable(false);
                            }
                        }
                    }
                });

                cols[1].vertical_centered_justified(|ui| {
                    if ui
                        .add(egui::Button::new("🚀 Start").min_size(egui::vec2(0.0, 40.0)))
                        .clicked()
                    {
                        manager.switch(PendingState::Overlay);
                        toasts.info("Starting subtitles overlay...")
                            .closable(false);
                    }
                });
            });
            ui.add_space(10.0);
        });
}

fn ui_log_level(ui: &mut Ui, settings: &mut SettingsApp) {
    ui.horizontal(|ui| {
        ui.label("Log Level:");
        ComboBox::from_id_salt("log_level")
            .selected_text(&settings.level)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut settings.level, "debug".to_string(), "Debug");
                ui.selectable_value(&mut settings.level, "info".to_string(), "Info");
                ui.selectable_value(&mut settings.level, "warn".to_string(), "Warn");
                ui.selectable_value(&mut settings.level, "error".to_string(), "Error");
            });
    });
}

fn ui_section_api(ui: &mut Ui, settings: &mut SettingsApp) {
    ui.collapsing("API Configuration", |ui| {
        Grid::new("api_grid")
            .num_columns(2)
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add(egui::Label::new("API Key:").extend());
                });
                ui.add(TextEdit::singleline(&mut settings.api_key).password(true));
                ui.end_row();

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add(egui::Label::new("Languages:").extend());
                });
                ui.vertical(|ui| {
                    let mut to_remove = None;
                    for (i, hint) in settings.language_hints.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("{}.", i + 1));
                            ComboBox::from_id_salt(format!("hint_{}", i))
                                .selected_text(hint.to_string())
                                .show_ui(ui, |ui| {
                                    for lang in LanguageHint::all() {
                                        ui.selectable_value(hint, *lang, lang.to_string());
                                    }
                                });
                            if ui.button("🗑").clicked() {
                                to_remove = Some(i);
                            }
                        });
                    }
                    if let Some(i) = to_remove {
                        settings.language_hints.remove(i);
                    }
                    if ui.button("➕ Add").clicked() {
                        settings.language_hints.push(LanguageHint::English);
                    }
                });
                ui.end_row();

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add(egui::Label::new("Translation:").extend());
                });
                ui.vertical(|ui| {
                    ui.checkbox(&mut settings.enable_translate, "Enable");
                    if settings.enable_translate {
                        ComboBox::from_id_salt("target_lang")
                            .selected_text(settings.target_language.to_string())
                            .show_ui(ui, |ui| {
                                for lang in LanguageHint::all() {
                                    ui.selectable_value(
                                        &mut settings.target_language,
                                        *lang,
                                        lang.to_string(),
                                    );
                                }
                            });
                    }
                });
                ui.end_row();
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add(egui::Label::new("Context:").extend());
                });
                ui.add(TextEdit::multiline(&mut settings.context).desired_rows(2));
                ui.end_row();
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add(egui::Label::new("Options:").extend());
                });
                ui.checkbox(&mut settings.enable_speakers, "Enable Speakers ID");
                ui.end_row();
            });
        ui.small("To change configuration Soniox you need restart application.");
    });
}

fn ui_section_position(ui: &mut Ui, ctx: &Context, settings: &mut SettingsApp) {
    ui.collapsing("Position", |ui| {
        Grid::new("pos_grid").spacing([10.0, 10.0]).show(ui, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.add(egui::Label::new("Coordinates:").extend());
            });
            ui.horizontal(|ui| {
                ui.label("X:");
                ui.add(DragValue::new(&mut settings.position.0));
                ui.label("Y:");
                ui.add(DragValue::new(&mut settings.position.1));
            });
            ui.end_row();

            ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                ui.add(egui::Label::new("Snap to:").extend());
            });
            ui.vertical(|ui| {
                let (screen_w, screen_h) = match ctx.input(|i| i.viewport().monitor_size) {
                    Some(size) => (size.x, size.y),
                    None => (1920.0, 1080.0),
                };

                let w = 800.0;
                let h = 350.0;
                let pad = 30.0;

                let x_left = pad;
                let x_center = (screen_w / 2.0) - (w / 2.0);
                let x_right = screen_w - w - pad;

                let y_top = pad;
                let y_mid = (screen_h / 2.0) - (h / 2.0);
                let y_bot = screen_h - h - pad;

                Grid::new("snap_buttons")
                    .spacing([5.0, 5.0])
                    .show(ui, |ui| {
                        let btn = |ui: &mut Ui, text: &str| {
                            ui.add(
                                egui::Button::new(RichText::new(text).size(16.0))
                                    .min_size(egui::vec2(30.0, 30.0)),
                            )
                        };

                        if btn(ui, "↖").clicked() {
                            settings.position = (x_left, y_top);
                        } else if btn(ui, "⬆").clicked() {
                            settings.position = (x_center, y_top);
                        } else if btn(ui, "↗").clicked() {
                            settings.position = (x_right, y_top);
                        }
                        ui.end_row();

                        if btn(ui, "⬅").clicked() {
                            settings.position = (x_left, y_mid);
                        } else if btn(ui, "◎").clicked() {
                            settings.position = (x_center, y_mid);
                        } else if btn(ui, "➡").clicked() {
                            settings.position = (x_right, y_mid);
                        }
                        ui.end_row();

                        if btn(ui, "↙")
                            .on_hover_text(format!("Y: {:.0}", y_bot))
                            .clicked()
                        {
                            settings.position = (x_left, y_bot);
                        } else if btn(ui, "⬇")
                            .on_hover_text(format!("Y: {:.0}", y_bot))
                            .clicked()
                        {
                            settings.position = (x_center, y_bot);
                        } else if btn(ui, "↘")
                            .on_hover_text(format!("Y: {:.0}", y_bot))
                            .clicked()
                        {
                            settings.position = (x_right, y_bot);
                        }
                        ui.end_row();
                    });
            });
            ui.end_row();
        });
    });
}

fn ui_section_appearance(ui: &mut Ui, settings: &mut SettingsApp) {
    ui.collapsing("Appearance", |ui| {
        Grid::new("appearance_grid")
            .spacing([10.0, 10.0])
            .show(ui, |ui| {
                ui.label("Max blocks:");
                ui.add(Slider::new(&mut settings.max_blocks, 1..=10));
                ui.end_row();

                ui.label("Font size:");
                ui.add(Slider::new(&mut settings.font_size, 10.0..=80.0));
                ui.end_row();

                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    ui.add(egui::Label::new("Style:").extend());
                });
                Grid::new("font_style").spacing([10.0, 8.0]).show(ui, |ui| {
                    ui.checkbox(&mut settings.enable_background, "Background");
                    ui.end_row();
                    ui.checkbox(&mut settings.enable_high_priority, "Always on top");
                    ui.end_row();
                });
                ui.end_row();
            });

        ui.separator();

        Grid::new("colors_grid")
            .num_columns(2)
            .spacing([10.0, 8.0])
            .show(ui, |ui| {
                ui.label("Text Color:");
                ui.horizontal(|ui| {
                    if ui
                        .button("↺ Reset")
                        .on_hover_text("Reset to Yellow")
                        .clicked()
                    {
                        settings.text_color = (255, 255, 0);
                    }
                });
                ui.end_row();

                ui.label("Red:");
                ui.add(Slider::new(&mut settings.text_color.0, 0..=255));
                ui.end_row();

                ui.label("Green:");
                ui.add(Slider::new(&mut settings.text_color.1, 0..=255));
                ui.end_row();

                ui.label("Blue:");
                ui.add(Slider::new(&mut settings.text_color.2, 0..=255));
                ui.end_row();
            });

        if ui.button("Reset Color").clicked() {
            settings.text_color = (255, 255, 0);
        }

        let p_col = Color32::from_rgb(
            settings.text_color.0,
            settings.text_color.1,
            settings.text_color.2,
        );
        egui::Frame::new()
            .fill(p_col)
            .corner_radius(5.0)
            .inner_margin(8.0)
            .show(ui, |ui| {
                let text_col = if (settings.text_color.0 as u16
                    + settings.text_color.1 as u16
                    + settings.text_color.2 as u16)
                    > 380
                {
                    Color32::BLACK
                } else {
                    Color32::WHITE
                };
                ui.label(
                    RichText::new(format!("Preview ({:.0}px)", settings.font_size))
                        .color(text_col)
                        .size(settings.font_size),
                );
            });
    });
}
