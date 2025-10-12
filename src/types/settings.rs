use crate::errors::SonioxWindowsErrors;
use config::{Config, ConfigError, File};
use log::LevelFilter;
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SettingsApp {
    pub language_hints: Vec<String>,
    pub context: String,
    pub api_key: String,
    pub level: String,
    // pub high_priority: bool,
}

impl SettingsApp {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        s.try_deserialize()
    }

    pub fn level(&self) -> Result<LevelFilter, SonioxWindowsErrors> {
        LevelFilter::from_str(&self.level).map_err(|_| {
            SonioxWindowsErrors::Internal(
                "field `level` isn't valid. did u mean `info`, `debug` and `warn`?",
            )
        })
    }
}
