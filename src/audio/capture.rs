use crate::errors::SonioxWindowsErrors;
use crate::types::audio::{AudioMessage, AudioSample};
use cpal::{Stream, StreamConfig};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use tokio::sync::mpsc::{Receiver, Sender};

pub fn start_capture_audio(
    tx_audio: Sender<AudioMessage>,
    mut rx_recycle: Receiver<AudioSample>,
) -> Result<(Stream, StreamConfig), SonioxWindowsErrors> {
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or_else(|| SonioxWindowsErrors::Internal("Output device is not found"))?;
    let config = device.default_output_config()?.config();
    let tx_audio_clone = tx_audio.clone();

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buffer = match rx_recycle.try_recv() {
                Ok(mut vec) => {
                    vec.clear();
                    vec
                }
                Err(_) => Vec::with_capacity(data.len()),
            };

            buffer.extend_from_slice(data);

            if let Err(e) = tx_audio_clone.blocking_send(AudioMessage::Audio(buffer)) {
                tracing::error!("Error in send audio: {}", e);
            }
        },
        |err| {
            tracing::error!("Error in audio callback: {}", err);
        },
        None,
    )?;

    stream.play()?;

    Ok((stream, config))
}
