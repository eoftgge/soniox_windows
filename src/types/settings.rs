use crate::errors::SonioxWindowsErrors;
use crate::types::languages::LanguageHint;
use config::{Config, ConfigError, File};
use log::LevelFilter;
use serde::Deserialize;
use std::str::FromStr;
use eframe::egui::{pos2, Color32, Pos2};

#[derive(Deserialize)]
pub struct SettingsApp {
    pub(crate) language_hints: Vec<LanguageHint>,
    pub(crate) context: String,
    pub(crate) api_key: String,
    pub(crate) target_language: LanguageHint,
    pub(crate) enable_translate: bool,
    enable_high_priority: bool,
    enable_speakers: bool,
    enable_background: bool,
    level: String,
    position: (f32, f32),
    font_size: f32,
    text_color: (u8, u8, u8),
}

impl SettingsApp {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        s.try_deserialize()
    }

    pub fn language_hints(&self) -> &[LanguageHint] {
        &self.language_hints
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn target_language(&self) -> LanguageHint {
        self.target_language
    }

    pub fn enable_speakers(&self) -> bool {
        self.enable_speakers
    }

    pub fn enable_translate(&self) -> bool {
        self.enable_translate
    }

    pub fn enable_high_priority(&self) -> bool {
        self.enable_high_priority
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn level(&self) -> Result<LevelFilter, SonioxWindowsErrors> {
        LevelFilter::from_str(&self.level).map_err(|_| {
            SonioxWindowsErrors::Internal(
                "field `level` isn't valid. did u mean `info`, `debug` and `warn`?",
            )
        })
    }

    pub fn text_color(&self) -> Color32 {
        Color32::from_rgb(self.text_color.0, self.text_color.1, self.text_color.2)
    }

    pub fn get_background_color(&self) -> Color32 {
        if self.enable_background {
            return Color32::from_black_alpha(155);
        }
        Color32::TRANSPARENT
    }

    pub fn get_position(&self) -> Pos2 {
        pos2(self.position.0, self.position.1)
    }
}
