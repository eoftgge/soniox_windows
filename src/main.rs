pub mod stream;
pub mod errors;
pub mod types;

use std::error::Error;
use std::thread;

use crossbeam_channel::unbounded;
use hound::{WavSpec, WavWriter, SampleFormat};
use wasapi::{initialize_mta, get_default_device, Direction};
use crate::stream::start_capture_audio;

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = unbounded::<Vec<u8>>();
    thread::spawn(move || start_capture_audio(tx));
    
    initialize_mta().ok().unwrap();
    let device = get_default_device(&Direction::Render).ok().unwrap();
    let audio_client = device.get_iaudioclient().ok().unwrap();
    let format = audio_client.get_mixformat().ok().unwrap();
    let sample_rate = format.get_samplespersec();
    let channels = format.get_nchannels();
    let bits_per_sample = format.get_bitspersample();

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
    println!("Запись, формат: {} Hz, {} ch, {} bits", sample_rate, channels, bits_per_sample);

    // for buf in rx.iter() {
    //     if bits_per_sample == 32 {
    //         let float_count = buf.len() / 4;
    //         let floats: &[f32] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const f32, float_count) };
    //         for &f in floats {
    //             writer.write_sample(f)?;
    //         }
    //     } else if bits_per_sample == 16 {
    //         let samples = buf.len() / 2;
    //         let i16s: &[i16] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const i16, samples) };
    //         for &s in i16s {
    //             writer.write_sample(s)?;
    //         }
    //     } else {
    //         let float_count = buf.len() / 4;
    //         let floats: &[f32] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const f32, float_count) };
    //         for &f in floats {
    //             let s = (f * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16;
    //             writer.write_sample(s)?;
    //         }
    //     }
    // }

    writer.finalize()?;

    while let Ok(data) = rx.recv() {
        println!("data {:?}", data);
    }

    println!("Готово: system_audio.wav");
    Ok(())
}
