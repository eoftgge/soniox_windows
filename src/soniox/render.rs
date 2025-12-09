use crate::types::audio::AudioSubtitle;
use crate::types::soniox::SonioxTranscriptionResponse;

pub(crate) fn render_transcription(resp: SonioxTranscriptionResponse) -> Vec<AudioSubtitle> {
    let mut results = Vec::new();
    let mut current_text = String::new();
    let mut current_speaker = String::new();

    for token in resp.tokens {
        log::debug!("token of response: {:?}", token);
        if token.translation_status.as_deref() == Some("original") {
            continue;
        }

        match token.speaker {
            Some(speaker) if speaker != current_speaker => {
                results.push(AudioSubtitle::Speaker(current_speaker, current_text));
                current_speaker = speaker;
                current_text = token.text;
            },
            _ => current_text.push_str(&token.text),
        }
    }

    if !current_text.is_empty() && current_speaker.is_empty() {
        results.push(AudioSubtitle::Text(current_text));
    } else {
        results.push(AudioSubtitle::Speaker(current_speaker, current_text));
    }

    results
}
