use serde::{Deserialize, Serialize};

pub type AudioSample = Vec<u8>;

#[derive(Debug, Deserialize, Default)]
pub struct SonioxTranslationObject {
    pub r#type: String,
    pub language_a: String,
    pub language_b: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionRequest {
    pub api_key: String,
    pub model: String,
    pub audio_format: String,
    pub num_channels: Option<u32>, // required for raw audio
    pub sample_rate: Option<u32>, // required for raw audio
    pub language_hints: Vec<String>, // required
    pub context: Option<String>,
    pub enable_speaker_diarization: Option<bool>,
    pub enable_language_identification: Option<bool>,
    pub enable_non_final_tokens: Option<bool>,
    pub enable_endpoint_detection: Option<bool>,
    pub client_reference_id: Option<String>,
    pub translation: Option<SonioxTranslationObject>,
}

#[derive(Debug, Serialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionToken {
    pub text: String,
    pub start_ms: Option<u32>,
    pub end_ms: Option<u32>,
    pub confidence: u32,
    pub is_final: bool,
    pub speaker: Option<String>,
    pub language: Option<String>,
    pub source_language: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionResponse {
    pub tokens: Vec<SonioxTranscriptionToken>,
    pub final_audio_proc_ms: u32,
    pub total_audio_proc_ms: u32,
    pub finished: Option<bool>,
}