pub type AudioSample = Vec<f32>;

#[derive(Debug)]
pub enum AudioMessage {
    Audio(AudioSample),
    Stop,
}
