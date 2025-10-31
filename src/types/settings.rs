use crate::errors::SonioxWindowsErrors;
use crate::types::offset::{OFFSET_WIDTH, WINDOW_HEIGHT};
use config::{Config, ConfigError, File};
use log::LevelFilter;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SettingsApp {
    pub(crate) language_hints: Vec<String>, // TODO: add check languages
    pub(crate) context: String,
    pub(crate) api_key: String,
    pub(crate) target_language: String, // same
    pub(crate) enable_translate: bool,
    enable_speakers: bool,
    level: String,
    position: (f32, f32),
}

impl SettingsApp {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        s.try_deserialize()
    }

    pub fn language_hints(&self) -> &[String] {
        &self.language_hints
    }

    pub fn context(&self) -> &str {
        &self.context
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn target_language(&self) -> &str {
        &self.target_language
    }

    pub fn enable_speakers(&self) -> bool {
        self.enable_speakers
    }

    pub fn enable_translate(&self) -> bool {
        self.enable_translate
    }

    pub fn level(&self) -> Result<LevelFilter, SonioxWindowsErrors> {
        LevelFilter::from_str(&self.level).map_err(|_| {
            SonioxWindowsErrors::Internal(
                "field `level` isn't valid. did u mean `info`, `debug` and `warn`?",
            )
        })
    }

    pub fn get_position(&self, height: usize) -> (f32, f32) {
        if self.position == (0., 0.) {
            let window_height = WINDOW_HEIGHT;
            let pos_x = OFFSET_WIDTH;
            let pos_y = height as f32 - window_height - 100.;

            return (pos_x, pos_y);
        }
        self.position
    }
}
