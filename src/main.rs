use std::error::Error;
use std::thread;

use crossbeam_channel::unbounded;
use soniox_windows::soniox::start_soniox_stream;
use soniox_windows::stream::start_capture_audio;
use soniox_windows::types::AudioSample;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = std::env::var("SONIOX_APIKEY")?;
    let (tx, rx) = unbounded::<AudioSample>();
    thread::spawn(move || start_capture_audio(tx));
    start_soniox_stream(rx, api_key).await?;

    Ok(())
}
