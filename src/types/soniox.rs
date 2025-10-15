use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Default)]
pub struct SonioxTranslationObject {
    pub r#type: String,
    pub language_a: Option<String>,
    pub language_b: Option<String>,
    pub target_language: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionRequest {
    pub api_key: String,
    pub model: String,
    pub audio_format: String,
    pub num_channels: Option<u32>,   // required for raw audio
    pub sample_rate: Option<u32>,    // required for raw audio
    pub language_hints: Vec<String>, // required
    pub context: Option<String>,
    pub enable_speaker_diarization: Option<bool>,
    pub enable_language_identification: Option<bool>,
    pub enable_non_final_tokens: Option<bool>,
    pub enable_endpoint_detection: Option<bool>,
    pub client_reference_id: Option<String>,
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
    pub language: Option<String>,
    pub source_language: Option<String>,
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
