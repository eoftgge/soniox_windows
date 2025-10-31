use crate::types::offset::{OFFSET_WIDTH, WINDOW_HEIGHT};

pub fn get_inner_size(position: (f32, f32), width: f32) -> (f32, f32) {
    (width - position.0 - OFFSET_WIDTH * 2., WINDOW_HEIGHT)
}