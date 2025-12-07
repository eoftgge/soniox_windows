use crate::errors::SonioxWindowsErrors;
use crate::types::audio::AudioMessage;
use bytemuck::cast_slice;
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc::UnboundedSender;
use wasapi::{Direction, StreamMode, DeviceEnumerator, initialize_mta};

pub fn start_capture_audio(
    tx_audio: UnboundedSender<AudioMessage>,
) -> Result<(), SonioxWindowsErrors> {
    initialize_mta()
        .ok()
        .map_err(|_| SonioxWindowsErrors::Internal(""))?;
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

    log::info!("Started audio stream!");
    loop {
        let frames = match capture.get_next_packet_size()? {
            Some(f) if f > 0 => f,
            _ => {
                sleep(Duration::from_millis(50));
                continue;
            }
        };

        let mut buffer = vec![0u8; frames as usize * bytes_per_frame];
        let _ = capture.read_from_device(&mut buffer)?;

        let final_buffer: Vec<f32> = if !buffer.len().is_multiple_of(4) {
            log::warn!("Buffer size not multiple of 4: {}", buffer.len());
            Vec::new()
        } else {
            cast_slice::<u8, f32>(&buffer).to_vec()
        };
        let result = tx_audio.send(AudioMessage::Audio(final_buffer));

        if result.is_err() {
            log::info!("Audio thread terminated");
            let _ = tx_audio.send(AudioMessage::Stop);
            audio_client.stop_stream()?;
            break Ok(());
        }
    }
}
