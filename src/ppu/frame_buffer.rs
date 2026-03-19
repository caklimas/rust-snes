use crate::ppu::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct FrameBuffer(pub [u16; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize]);

impl Default for FrameBuffer {
    fn default() -> Self {
        Self([0; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize])
    }
}
