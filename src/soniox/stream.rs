use crate::errors::SonioxWindowsErrors;
use crate::soniox::render::render_transcription;
use crate::types::audio::AudioMessage;
use crate::types::settings::SettingsApp;
use crate::types::soniox::{SonioxTranscriptionRequest, SonioxTranscriptionResponse, SonioxTranslationObject};
use futures_util::{SinkExt, StreamExt};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::connect_async;
use tungstenite::client::IntoClientRequest;
use tungstenite::{Bytes, Message, Utf8Bytes};
use wasapi::{Direction, get_default_device, initialize_mta};

const URL: &str = "wss://stt-rt.soniox.com/transcribe-websocket";
const MODEL: &str = "stt-rt-v3";

fn create_request(settings: SettingsApp) -> SonioxTranscriptionRequest {
    initialize_mta().ok().unwrap();
    let device = get_default_device(&Direction::Render).ok().unwrap();
    let audio_client = device.get_iaudioclient().ok().unwrap();
    let format = audio_client.get_mixformat().ok().unwrap();
    let sample_rate = format.get_samplespersec();
    let channels = format.get_nchannels();
    let mut request = SonioxTranscriptionRequest {
        api_key: settings.api_key,
        model: MODEL.into(),
        audio_format: "pcm_s16le".into(),
        sample_rate: Some(sample_rate),
        num_channels: Some(channels as u32),
        context: Some(settings.context),
        language_hints: settings.language_hints,
        ..Default::default()
    };
    if settings.is_translate {
        request.translation = Some(SonioxTranslationObject {
            r#type: "one_way".into(),
            target_language: Some(settings.target_language),
            ..Default::default()
        });
    }

    request
}

async fn listen_soniox_stream(
    bytes: Vec<u8>,
    tx_subs: UnboundedSender<String>,
    mut rx_audio: UnboundedReceiver<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    'stream: loop {
        let url = URL.into_client_request()?;
        let (ws_stream, _) = connect_async(url).await?;
        let (mut write, mut read) = ws_stream.split();
        write
            .send(Message::Text(Utf8Bytes::try_from(bytes.clone())?))
            .await?;

        let tx_subs = tx_subs.clone();
        let reader = async move {
            while let Some(msg) = read.next().await {
                if let Message::Text(txt) = msg? {
                    let v: SonioxTranscriptionResponse = serde_json::from_str(&txt)?;
                    let t = render_transcription(&v);
                    let _ = tx_subs.send(t);
                }
            }
            <Result<(), SonioxWindowsErrors>>::Ok(())
        };

        tokio::spawn(async move {
            if let Err(err) = reader.await {
                log::error!("error during read message {}", err);
            }
        });
        while let Some(message) = rx_audio.recv().await {
            match message {
                AudioMessage::Audio(buffer) => {
                    if buffer.is_empty() {
                        break;
                    }
                    let pcm16: Vec<u8> = buffer
                        .iter()
                        .map(|&s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
                        .flat_map(|s| s.to_le_bytes())
                        .collect();

                    let result = write.send(Message::Binary(Bytes::from(pcm16))).await;

                    if let Err(err) = result {
                        log::error!("error during sent binary -> {:?}", err);
                        continue 'stream;
                    }
                }
                AudioMessage::Stop => {
                    let _ = write.send(Message::Binary(Bytes::new())).await;
                    break 'stream;
                }
            }
        }

        let result = write.send(Message::Binary(Bytes::new())).await;
        if let Err(err) = result {
            log::error!("error during sent empty binary -> {:?}", err);
            break;
        }
    }

    Ok(())
}

pub async fn start_soniox_stream(
    settings: SettingsApp,
    tx_subs: UnboundedSender<String>,
    rx_audio: UnboundedReceiver<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    let request = create_request(settings);
    let bytes = serde_json::to_vec(&request)?;

    log::info!("Started Soniox stream!");
    log::info!("Starting to listen websocket stream Soniox...");
    listen_soniox_stream(bytes, tx_subs, rx_audio).await
}
