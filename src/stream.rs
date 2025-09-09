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
        buffer_duration_hns: bytes_per_frame as i64,
    };
    audio_client
        .initialize_client(&format, &Direction::Capture, &mode)
        .ok()
        .unwrap();

    let capture = audio_client.get_audiocaptureclient().ok().unwrap();
    audio_client.start_stream().unwrap();

    loop {
        match capture.get_next_packet_size() {
            Ok(Some(frames)) => {
                if frames == 0 {
                    sleep(Duration::from_millis(5));
                    continue;
                }
                let bytes = (frames as usize).saturating_mul(bytes_per_frame);
                let mut buf = vec![0u8; bytes];

                let (frames_read, _info) = capture.read_from_device(&mut buf).unwrap();
                let bytes_read = (frames_read as usize).saturating_mul(bytes_per_frame);
                buf.truncate(bytes_read);

                tx.send(buf).unwrap();
            }
            Ok(None) => {
                if let Ok(pad_frames) = audio_client.get_current_padding() {
                    if pad_frames == 0 {
                        sleep(Duration::from_millis(5));
                        continue;
                    }
                    let bytes = (pad_frames as usize).saturating_mul(bytes_per_frame);
                    let mut buf = vec![0u8; bytes];
                    let (frames_read, _info) = capture.read_from_device(&mut buf).ok().unwrap();
                    let bytes_read = (frames_read as usize).saturating_mul(bytes_per_frame);
                    buf.truncate(bytes_read);
                    tx.send(buf).unwrap();
                } else {
                    sleep(Duration::from_millis(5));
                    continue;
                }
            }
            Err(e) => {
                eprintln!("get_next_packet_size error: {:?}", e);
                sleep(Duration::from_millis(10));
            }
        }
    }
}
