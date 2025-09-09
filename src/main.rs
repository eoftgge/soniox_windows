pub mod stream;
pub mod errors;
pub mod types;

use std::error::Error;
use std::time::{Duration, Instant};
use std::thread;

use crossbeam_channel::unbounded;
use hound::{WavSpec, WavWriter, SampleFormat};
use wasapi::{initialize_mta, get_default_device, Direction, StreamMode};

fn main() -> Result<(), Box<dyn Error>> {
    const RECORD_SECONDS: u64 = 5;

    let (tx, rx) = unbounded::<Vec<u8>>();

    let capture_handle = thread::spawn(move || {
        initialize_mta().ok().unwrap();
        let device = get_default_device(&Direction::Render).ok().unwrap();
        let mut audio_client = device.get_iaudioclient().ok().unwrap();
        let format = audio_client.get_mixformat().ok().unwrap();

        let mode = StreamMode::PollingShared {
            autoconvert: false,
            buffer_duration_hns: RECORD_SECONDS as i64,
        };
        audio_client.initialize_client(&format, &Direction::Capture, &mode).ok().unwrap();
        let format = audio_client.get_mixformat().ok().unwrap();
        let bytes_per_frame = format.get_blockalign() as usize;

        let capture = audio_client.get_audiocaptureclient().ok().unwrap();
        audio_client.start_stream().unwrap();

        let start = Instant::now();
        while start.elapsed() < Duration::from_secs(RECORD_SECONDS) {
            match capture.get_next_packet_size() {
                Ok(Some(frames)) => {
                    if frames == 0 {
                        thread::sleep(Duration::from_millis(5));
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
                            thread::sleep(Duration::from_millis(5));
                            continue;
                        }
                        let bytes = (pad_frames as usize).saturating_mul(bytes_per_frame);
                        let mut buf = vec![0u8; bytes];
                        let (frames_read, _info) = capture.read_from_device(&mut buf).ok().unwrap();
                        let bytes_read = (frames_read as usize).saturating_mul(bytes_per_frame);
                        buf.truncate(bytes_read);
                        tx.send(buf).unwrap();
                    } else {
                        thread::sleep(Duration::from_millis(5));
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("get_next_packet_size error: {:?}", e);
                    thread::sleep(Duration::from_millis(10));
                }
            }
        }

        audio_client.stop_stream().ok();
    });
    
    initialize_mta().ok().unwrap();
    let device = get_default_device(&Direction::Render).ok().unwrap();
    let audio_client = device.get_iaudioclient().ok().unwrap();
    let format = audio_client.get_mixformat().ok().unwrap();
    let sample_rate = format.get_samplespersec();
    let channels = format.get_nchannels();
    let bits_per_sample = format.get_bitspersample();
    let bytes_per_frame = format.get_blockalign();

    let sample_format = if bits_per_sample == 32 {
        SampleFormat::Float
    } else {
        SampleFormat::Int
    };

    let spec = WavSpec {
        channels,
        sample_rate,
        bits_per_sample,
        sample_format,
    };

    let mut writer = WavWriter::create("system_audio.wav", spec)?;
    println!("Запись {} сек — формат: {} Hz, {} ch, {} bits", RECORD_SECONDS, sample_rate, channels, bits_per_sample);

    for buf in rx.iter() {
        if bits_per_sample == 32 {
            let float_count = buf.len() / 4;
            let floats: &[f32] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const f32, float_count) };
            for &f in floats {
                writer.write_sample(f)?;
            }
        } else if bits_per_sample == 16 {
            let samples = buf.len() / 2;
            let i16s: &[i16] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const i16, samples) };
            for &s in i16s {
                writer.write_sample(s)?;
            }
        } else {
            let float_count = buf.len() / 4;
            let floats: &[f32] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const f32, float_count) };
            for &f in floats {
                let s = (f * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
                writer.write_sample(s)?;
            }
        }
    }

    writer.finalize()?;
    capture_handle.join().expect("capture thread panicked");

    println!("Готово: system_audio.wav");
    Ok(())
}
