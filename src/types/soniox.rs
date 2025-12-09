use serde::{Deserialize, Serialize};
use crate::types::languages::LanguageHint;

#[derive(Debug, Serialize, Default)]
pub struct SonioxTranslationObject {
    pub r#type: &'static str,
    pub language_a: Option<LanguageHint>,
    pub language_b: Option<LanguageHint>,
    pub target_language: Option<LanguageHint>,
}

#[derive(Debug, Serialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionRequest<'a> {
    pub api_key: &'a str,
    pub model: &'static str,
    pub audio_format: &'static str,
    pub num_channels: Option<u32>,    // required for raw audio
    pub sample_rate: Option<u32>,     // required for raw audio
    pub language_hints: &'a [LanguageHint], // required
    pub context: Option<&'a str>,
    pub enable_speaker_diarization: Option<bool>,
    pub enable_language_identification: Option<bool>,
    pub enable_non_final_tokens: Option<bool>,
    pub enable_endpoint_detection: Option<bool>,
    pub client_reference_id: Option<&'a str>,
    pub translation: Option<SonioxTranslationObject>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionToken {
    pub text: String,
    pub start_ms: Option<f64>,
    pub end_ms: Option<f64>,
    pub confidence: f64,
    pub is_final: bool,
    pub speaker: Option<String>,
    pub language: Option<LanguageHint>,
    pub source_language: Option<LanguageHint>,
    pub translation_status: Option<String>, // maybe add enum?
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionResponse {
    pub tokens: Vec<SonioxTranscriptionToken>,
    pub final_audio_proc_ms: f64,
    pub total_audio_proc_ms: f64,
    pub finished: Option<bool>,
}
