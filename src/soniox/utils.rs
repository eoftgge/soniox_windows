pub fn convert_audio_chunk(input: &[f32], output: &mut Vec<u8>) {
    output.clear();

    let required_len = input.len() * 2;
    output.reserve(required_len);

    const SCALE: f32 = i16::MAX as f32;

    output.extend(input.iter().flat_map(|&s| {
        let sample = (s.clamp(-1.0, 1.0) * SCALE) as i16;
        sample.to_le_bytes()
    }));
}