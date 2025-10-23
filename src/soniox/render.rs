use crate::types::audio::AudioSubtitle;
use crate::types::soniox::SonioxTranscriptionResponse;


pub(crate) fn render_transcription(resp: &SonioxTranscriptionResponse) -> Vec<AudioSubtitle> {
    let mut results = Vec::new();
    let mut current_text = String::new();
    let mut prev_speaker: Option<String> = None;

    for token in &resp.tokens {
        if token.translation_status.as_deref() == Some("original") {
            continue;
        }
        let current_speaker = token.speaker.clone();

        if prev_speaker != current_speaker && !current_text.is_empty() {
            results.push(AudioSubtitle::Text(current_text.trim().to_string()));
            current_text = String::new();
        }

        current_text.push_str(&token.text);
        prev_speaker = current_speaker;
    }

    if !current_text.is_empty() {
        results.push(AudioSubtitle::Text(current_text.trim().to_string()));
    }

    results
}
