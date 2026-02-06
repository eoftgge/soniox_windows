use crate::errors::SonioxLiveErrors;
use crate::types::languages::LanguageHint;
use eframe::egui::{Color32, Pos2, pos2};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use tracing_subscriber::filter::LevelFilter;

#[derive(Deserialize, Serialize, Clone)]
pub struct SettingsApp {
    pub(crate) language_hints: Vec<LanguageHint>,
    pub(crate) context: String,
    pub(crate) api_key: String,
    pub(crate) target_language: LanguageHint,
    pub(crate) enable_translate: bool,
    pub(crate) enable_high_priority: bool,
    pub(crate) enable_speakers: bool,
    pub(crate) enable_background: bool,
    pub(crate) level: String, // maybe to make it an enum
    pub(crate) position: (f32, f32),
    pub(crate) font_size: usize,
    pub(crate) text_color: (u8, u8, u8),
    pub(crate) max_blocks: usize,
}

impl Default for SettingsApp {
    fn default() -> Self {
        Self {
            language_hints: vec![LanguageHint::default()],
            context: String::from("some kind context"),
            api_key: String::new(),
            target_language: LanguageHint::default(),
            enable_translate: false,
            enable_high_priority: true,
            enable_speakers: true,
            enable_background: true,
            level: "info".into(),
            position: (100., 100.),
            font_size: 18,
            text_color: (255, 255, 0), // yellow
            max_blocks: 3,
        }
    }
}

impl SettingsApp {
    pub fn new(path: &str) -> Result<Self, SonioxLiveErrors> {
        let path = Path::new(path);
        if !path.exists() {
            let s = Self::default();
            let content = toml::to_string(&s)?;
            std::fs::write(path, content)?;
            return Ok(s);
        }

        let content = std::fs::read_to_string(path)?;
        let s = toml::from_str(&content)?;
        Ok(s)
    }

    pub fn language_hints(&self) -> Arc<[LanguageHint]> {
        Arc::from(&*self.language_hints)
    }

    pub fn context(&self) -> Arc<str> {
        Arc::from(&*self.context)
    }

    pub fn api_key(&self) -> Arc<str> {
        Arc::from(&*self.api_key)
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
        self.font_size as f32
    }

    pub fn max_blocks(&self) -> usize {
        self.max_blocks
    }

    pub fn level(&self) -> Result<LevelFilter, SonioxLiveErrors> {
        LevelFilter::from_str(&self.level).map_err(|_| {
            SonioxLiveErrors::from(
                "field `level` isn't valid. did u mean `trace`, `debug` and `warn`?",
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

    pub fn save(&self, path: &str) -> Result<(), SonioxLiveErrors> {
        let toml_string = toml::to_string(self)?;
        std::fs::write(path, toml_string)?;

        Ok(())
    }
}
