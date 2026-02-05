use crate::errors::SonioxWindowsErrors;
use crate::types::audio::{AudioMessage, AudioSample};
use bytemuck::cast_slice;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::{Receiver, Sender};
use wasapi::{DeviceEnumerator, Direction, StreamMode, initialize_mta};

pub fn start_capture_audio(
    tx_audio: Sender<AudioMessage>,
    mut rx_stop: Receiver<bool>,
    mut rx_recycle: Receiver<AudioSample>,
) -> Result<(), SonioxWindowsErrors> {
    initialize_mta()
        .ok()
        .map_err(|_| SonioxWindowsErrors::Internal("Failed to init MTA"))?;

    let enumerator = DeviceEnumerator::new()?;
    let device = enumerator.get_default_device(&Direction::Render)?;
    let mut audio_client = device.get_iaudioclient()?;
    let format = audio_client.get_mixformat()?;
    let bytes_per_frame = format.get_blockalign() as usize;

    let mode = StreamMode::PollingShared {
        autoconvert: false,
        buffer_duration_hns: 1_000_000,
    };
    audio_client.initialize_client(&format, &Direction::Capture, &mode)?;

    let capture = audio_client.get_audiocaptureclient()?;
    audio_client.start_stream()?;

    let mut raw_buffer: Vec<u8> = Vec::with_capacity(16 * 1024);
    tracing::info!("Started audio stream!");
    loop {
        if let Ok(true) = rx_stop.try_recv() {
            tracing::info!("Audio thread terminated!");
            break;
        }

        let frames_available = match capture.get_next_packet_size() {
            Ok(Some(f)) => f,
            Err(e) => {
                tracing::error!("Error getting packet size: {:?}", e);
                continue;
            }
            _ => {
                tracing::error!("Unknown error in `get_next_packet_size`");
                break;
            }
        };

        if frames_available == 0 {
            sleep(Duration::from_millis(1));
            continue;
        }

        let bytes_needed = frames_available as usize * bytes_per_frame;
        if raw_buffer.len() != bytes_needed {
            raw_buffer.resize(bytes_needed, 0);
        }
        if let Err(e) = capture.read_from_device(&mut raw_buffer) {
            tracing::warn!("Read error: {:?}", e);
            continue;
        }
        let mut buffer = match rx_recycle.try_recv() {
            Ok(mut vec) => {
                vec.clear();
                vec
            }
            Err(_) => Vec::with_capacity(raw_buffer.len() / 4),
        };

        let data: &[f32] = cast_slice(&raw_buffer);
        buffer.extend_from_slice(data);

        match tx_audio.try_send(AudioMessage::Audio(buffer)) {
            Err(tokio::sync::mpsc::error::TrySendError::Full(_)) => {
                tracing::warn!("Audio buffer full, dropping frame");
                continue;
            },
            Err(tokio::sync::mpsc::error::TrySendError::Closed(_)) => {
                break;
            },
            _ => {}
        }
    }

    audio_client.stop_stream()?;
    let _ = tx_audio.send(AudioMessage::Stop);
    Ok(())
}
