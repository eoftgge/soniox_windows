use serde::Deserialize;
use config::{Config, ConfigError, File};

#[derive(Deserialize)]
pub struct SettingsApp {
    pub language_hints: Vec<String>,
    pub context: String,
    pub api_key: String,
    // pub level: String,
    // pub high_priority: bool,
}

impl SettingsApp {
    pub fn new(path: &str) -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(path))
            .build()?;
        s.try_deserialize()
    }
}