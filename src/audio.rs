use crate::errors::SonioxWindowsErrors;
use crate::types::AudioMessage;
use tokio::sync::mpsc::UnboundedSender;
use std::thread::sleep;
use std::time::Duration;
use wasapi::{Direction, StreamMode, get_default_device, initialize_mta};

pub fn start_capture_audio(tx_audio: UnboundedSender<AudioMessage>) -> Result<(), SonioxWindowsErrors> {
    initialize_mta()
        .ok()
        .or_else(|_| Err(SonioxWindowsErrors::Internal("")))?;
    let device = get_default_device(&Direction::Render)?;
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
                sleep(Duration::from_millis(5));
                continue;
            }
        };

        let mut buffer = vec![0u8; frames as usize * bytes_per_frame];
        let _ = capture.read_from_device(&mut buffer)?;

        let final_buffer: Vec<f32> = buffer
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();
        let result = tx_audio.send(AudioMessage::Audio(final_buffer));

        if result.is_err() {
            log::info!("Audio thread terminated");
            break Ok(());
        }
    }
}
