use std::error::Error;
use std::thread;

use crossbeam_channel::unbounded;
use hound::{SampleFormat, WavSpec, WavWriter};
use soniox_windows::stream::start_capture_audio;
use wasapi::{Direction, get_default_device, initialize_mta};

fn main() -> Result<(), Box<dyn Error>> {
    let (tx, rx) = unbounded::<Vec<u8>>();
    thread::spawn(move || start_capture_audio(tx));
    
    Ok(())
}
