use std::f32;
use futures_util::StreamExt;
use crossbeam_channel::{Receiver, Sender};
use futures_util::SinkExt;
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::{Bytes, Message, Utf8Bytes};
use wasapi::{get_default_device, initialize_mta, Direction};
use crate::errors::SonioxWindowsErrors;
use crate::types::{AudioSample, SonioxTranscriptionRequest, SonioxTranscriptionResponse};

fn render_transcription(resp: &SonioxTranscriptionResponse) -> String {
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

fn create_request(api_key: String) -> SonioxTranscriptionRequest {
    initialize_mta().ok().unwrap();
    let device = get_default_device(&Direction::Render).ok().unwrap();
    let audio_client = device.get_iaudioclient().ok().unwrap();
    let format = audio_client.get_mixformat().ok().unwrap();
    let sample_rate = format.get_samplespersec();
    let channels = format.get_nchannels();
    SonioxTranscriptionRequest {
        api_key,
        model: "stt-rt-preview-v2".into(),
        audio_format: "pcm_s16le".into(),
        sample_rate: Some(sample_rate),
        num_channels: Some(channels as u32),
        language_hints: vec!["en".into(), "ru".into()],
        ..Default::default()
    }
}

pub async fn start_soniox_stream(rx: Receiver<AudioSample>, api_key: String, tx_text: Sender<String>) -> Result<(), SonioxWindowsErrors> {
    let request = create_request(api_key);
    let bytes = serde_json::to_vec(&request)?;
    let url = "wss://stt-rt.soniox.com/transcribe-websocket".into_client_request()?;
    let (ws_stream, _) = connect_async(url).await?;

    let (mut write, mut read) = ws_stream.split();
    write.send(Message::Text(Utf8Bytes::try_from(bytes).unwrap())).await?;

    tokio::spawn(async move {
        while let Ok(buf) = rx.recv() {
            if buf.is_empty() { break; }
            let pcm16: Vec<u8> = buf
                .iter()
                .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                .flat_map(|s| s.to_le_bytes())
                .collect();

            write.send(Message::Binary(Bytes::from(pcm16))).await.unwrap();
        }

        write.send(Message::Binary(Bytes::new())).await.unwrap();
    });

    while let Some(msg) = read.next().await {
        match msg? {
            Message::Text(txt) => {
                let v: SonioxTranscriptionResponse = serde_json::from_str(&txt)?;
                let s = render_transcription(&v);
                let _ = tx_text.send(s);
            }
            _ => {}
        }
    }
    Ok(())
}