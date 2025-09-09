use std::sync::Arc;
use serde::{Deserialize, Serialize};

pub type AudioSample = Vec<u8>;

#[derive(Debug, Deserialize, Default)]
pub struct SonioxTranslationObject {
    r#type: String,
    language_a: String,
    language_b: String,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionRequest {
    api_key: String,
    model: String,
    audio_format: String,
    num_channels: Option<u32>, // required for raw audio
    sample_rate: Option<u32>, // required for raw audio
    language_hints: Vec<String>, // required
    context: Option<String>,
    enable_speaker_diarization: Option<bool>,
    enable_language_identification: Option<bool>,
    enable_non_final_tokens: Option<bool>,
    enable_endpoint_detection: Option<bool>,
    client_reference_id: Option<String>,
    translation: Option<SonioxTranslationObject>,
}

#[derive(Debug, Serialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionToken {
    text: String,
    start_ms: Option<u32>,
    end_ms: Option<u32>,
    confidence: u32,
    is_final: bool,
    speaker: Option<String>,
    language: Option<String>,
    source_language: Option<String>,
}

#[derive(Debug, Serialize, Default)]
#[serde(default)]
pub struct SonioxTranscriptionResponse {
    tokens: Vec<SonioxTranscriptionToken>,
    final_audio_proc_ms: u32,
    total_audio_proc_ms: u32,
    finished: Option<bool>,
}