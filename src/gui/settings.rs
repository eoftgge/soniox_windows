use std::time::Duration;
use eframe::egui::{self, ComboBox, Context, DragValue, Grid, RichText, ScrollArea, Slider, TextEdit};
use eframe::epaint::Color32;
use egui_notify::Toasts;
use crate::gui::state::AppState;
use crate::settings::SettingsApp;
use crate::types::languages::LanguageHint;

pub fn show_settings_window(
    ctx: &Context,
    settings: &mut SettingsApp,
    state: &mut AppState,
    toasts: &mut Toasts,
) {
    egui::TopBottomPanel::bottom("settings_bottom_panel")
        .resizable(false)
        .min_height(60.0)
        .show(ctx, |ui| {
            ui.add_space(15.0);
            ui.columns(2, |cols| {
                cols[0].vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new("💾 Save").min_size(egui::vec2(0.0, 40.0))).clicked() {
                        match settings.save("soniox.toml") {
                            Ok(_) => {
                                toasts.success("Settings saved successfully!")
                                    .duration(Duration::from_secs(3));
                            },
                            Err(e) => {
                                toasts.error(format!("Failed to save: {}", e))
                                    .duration(Duration::from_secs(5));
                            },
                        }
                    }
                });

                cols[1].vertical_centered_justified(|ui| {
                    if ui.add(egui::Button::new("🚀 Start").min_size(egui::vec2(0.0, 40.0))).clicked() {
                        let _ = settings.save("soniox.toml");
                        *state = AppState::Overlay;

                        toasts.info("Starting subtitles overlay...");
                    }
                });
            });
            ui.add_space(10.0);
        });

    egui::CentralPanel::default()
        .frame(egui::Frame::central_panel(&ctx.style()).inner_margin(15.0))
        .show(ctx, |ui| {
        ui.spacing_mut().item_spacing = egui::vec2(8.0, 12.0);
        ui.heading("Settings");
        ui.separator();

        ScrollArea::vertical().show(ui, |ui| {
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
                                    if ui.button("🗑").clicked() { to_remove = Some(i); }
                                });
                            }
                            if let Some(i) = to_remove { settings.language_hints.remove(i); }
                            if ui.button("➕ Add").clicked() { settings.language_hints.push(LanguageHint::English); }
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
                                            ui.selectable_value(&mut settings.target_language, *lang, lang.to_string());
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
                        ui.checkbox(&mut settings.enable_speakers, "Speakers ID");
                        ui.end_row();
                        ui.small("To change configuration Soniox you need restart application.")
                    });
            });

            ui.collapsing("Position", |ui| {
                Grid::new("pos_grid").spacing([10.0, 10.0]).show(ui, |ui| {
                    ui.label("Coordinates:");
                    ui.horizontal(|ui| {
                        ui.label("X:"); ui.add(DragValue::new(&mut settings.position.0));
                        ui.label("Y:"); ui.add(DragValue::new(&mut settings.position.1));
                    });
                    ui.end_row();

                    ui.label("Auto-set:");
                    let btn = ui.button("Use current position");
                    if btn.clicked() && let Some(rect) = ctx.input(|i| i.viewport().outer_rect) {
                        settings.position = (rect.min.x, rect.min.y);
                    }
                    btn.on_hover_text("Drag window to location and click");
                    ui.end_row();
                });
            });

            ui.collapsing("Appearance", |ui| {
                Grid::new("appearance_grid").spacing([10.0, 10.0]).show(ui, |ui| {
                    ui.label("Max blocks:");
                    ui.add(Slider::new(&mut settings.max_blocks, 1..=10));
                    ui.end_row();

                    ui.label("Font size:");
                    ui.add(Slider::new(&mut settings.font_size, 10.0..=80.0));
                    ui.end_row();

                    ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                        ui.add(egui::Label::new("Style:").extend());
                    });
                    Grid::new("font_style")
                        .spacing([10.0, 8.0]).show(ui, |ui| {
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
                            if ui.button("↺ Reset").on_hover_text("Reset to Yellow").clicked() {
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

                if ui.button("Reset Color").clicked() { settings.text_color = (255, 255, 0); }

                let p_col = Color32::from_rgb(settings.text_color.0, settings.text_color.1, settings.text_color.2);
                egui::Frame::new().fill(p_col).corner_radius(5.0).inner_margin(8.0).show(ui, |ui| {
                    let text_col = if (settings.text_color.0 as u16 + settings.text_color.1 as u16 + settings.text_color.2 as u16) > 380 { Color32::BLACK } else { Color32::WHITE };
                    ui.label(RichText::new(format!("Preview ({:.0}px)", settings.font_size)).color(text_col).size(settings.font_size));
                });
            });

            ui.allocate_space(egui::vec2(0.0, 60.0));
        });
    });
}