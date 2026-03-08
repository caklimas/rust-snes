use crate::ppu::bg_mode::BgMode;

pub struct PaletteBase {
    pub bg1: u8,
    pub bg2: u8,
    pub bg3: u8,
    pub bg4: u8,
}

impl PaletteBase {
    pub fn new(bg_mode: &BgMode) -> Self {
        match bg_mode.bg_mode() {
            0 => Self {
                bg1: 0,
                bg2: 32,
                bg3: 64,
                bg4: 96,
            },
            1..=3 => Self {
                bg1: 0,
                bg2: 0,
                bg3: 0,
                bg4: 0,
            },
            _ => unimplemented!(),
        }
    }
}
