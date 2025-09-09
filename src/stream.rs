use crate::types::AudioSample;
use crossbeam_channel::Sender;
use std::thread::sleep;
use std::time::Duration;
use wasapi::{Direction, StreamMode, get_default_device, initialize_mta};

pub fn start_capture_audio(tx: Sender<AudioSample>) {
    initialize_mta().ok().unwrap();
    let device = get_default_device(&Direction::Render).ok().unwrap();
    let mut audio_client = device.get_iaudioclient().ok().unwrap();
    let format = audio_client.get_mixformat().ok().unwrap();
    let bytes_per_frame = format.get_blockalign() as usize;

    let mode = StreamMode::PollingShared {
        autoconvert: false,
        buffer_duration_hns: 1_000_000,
    };
    audio_client
        .initialize_client(&format, &Direction::Capture, &mode)
        .ok()
        .unwrap();

    let capture = audio_client.get_audiocaptureclient().ok().unwrap();
    audio_client.start_stream().unwrap();

    loop {
        let frames = match capture.get_next_packet_size().unwrap() {
            Some(f) if f > 0 => f,
            _ => { sleep(Duration::from_millis(5)); continue; }
        };

        let mut buf = vec![0u8; frames as usize * bytes_per_frame];
        let _ = capture.read_from_device(&mut buf).unwrap();

        // float32 стерео
        let float_buf: Vec<f32> = buf
            .chunks_exact(4)
            .map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        let _ = tx.send(float_buf);
    }
}
