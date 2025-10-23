use crate::types::audio::AudioSubtitle;
use crate::types::soniox::SonioxTranscriptionResponse;

pub(crate) fn render_transcription(resp: &SonioxTranscriptionResponse) -> AudioSubtitle {
    let mut final_translation = String::new();
    let mut interim_translation = String::new();
    let mut final_original = String::new();
    let mut interim_original = String::new();

    let mut has_any_translation = false;
    let mut speaker_name: Option<String> = None;

    for token in &resp.tokens {
        if let Some(spk) = &token.speaker {
            speaker_name = Some(spk.clone());
        }

        match token.translation_status.as_deref() {
            Some("translation") => {
                has_any_translation = true;
                if token.is_final {
                    final_translation.push_str(&token.text);
                } else {
                    interim_translation.push_str(&token.text);
                }
            }
            _ if !has_any_translation => {
                if token.is_final {
                    final_original.push_str(&token.text);
                } else {
                    interim_original.push_str(&token.text);
                }
            }
            _ => {}
        }
    }

    if final_translation.is_empty()
        && interim_translation.is_empty()
        && final_original.is_empty()
        && interim_original.is_empty()
    {
        return AudioSubtitle::Empty;
    }

    let text = if has_any_translation {
        if !interim_translation.is_empty() {
            format!("{}{}", final_translation, interim_translation)
        } else {
            final_translation
        }
    } else if !interim_original.is_empty() {
        format!("{}{}", final_original, interim_original)
    } else {
        final_original
    };

    match speaker_name {
        Some(name) if !name.is_empty() => AudioSubtitle::Speaker(name, text),
        _ => AudioSubtitle::Text(text),
    }
}
