use crate::types::soniox::SonioxTranscriptionResponse;

pub(crate) fn render_transcription(resp: &SonioxTranscriptionResponse) -> String {
    let mut final_translation = String::new();
    let mut interim_translation = String::new();
    let mut final_original = String::new();
    let mut interim_original = String::new();

    let mut has_translation = false;

    for token in &resp.tokens {
        match token.translation_status.as_deref() {
            Some("translation") => {
                has_translation = true;
                if token.is_final {
                    final_translation.push_str(&token.text);
                } else {
                    interim_translation.push_str(&token.text);
                }
            }
            Some("original") => if !has_translation { has_translation = true; }
            _ => {
                if !has_translation {
                    if token.is_final {
                        final_original.push_str(&token.text);
                    } else {
                        interim_original.push_str(&token.text);
                    }
                }
            }
        }
    }

    if has_translation {
        if !interim_translation.is_empty() {
            format!("{}{}", final_translation, interim_translation)
        } else {
            final_translation
        }
    } else if !interim_original.is_empty() {
        format!("{}{}", final_original, interim_original)
    } else {
        final_original
    }
}