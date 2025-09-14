use crate::types::soniox::SonioxTranscriptionResponse;

pub(crate) fn render_transcription(resp: &SonioxTranscriptionResponse) -> String {
    let mut final_text = String::new();
    let mut interim_text = String::new();

    for token in &resp.tokens {
        if token.is_final {
            final_text.push_str(&token.text);
        } else {
            interim_text.push_str(&token.text);
        }
    }

    if !interim_text.is_empty() {
        format!("{}{}", final_text, interim_text)
    } else {
        final_text
    }
}
