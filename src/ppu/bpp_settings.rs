use crate::ppu::bg_mode::BgMode;

pub struct BppSettings {
    pub bg1: Option<u8>,
    pub bg2: Option<u8>,
    pub bg3: Option<u8>,
    pub bg4: Option<u8>,
}

impl BppSettings {
    pub fn new(bg_mode: &BgMode) -> Self {
        match bg_mode.bg_mode() {
            0 => Self {
                bg1: Some(2),
                bg2: Some(2),
                bg3: Some(2),
                bg4: Some(2),
            },
            1 => Self {
                bg1: Some(4),
                bg2: Some(4),
                bg3: Some(2),
                bg4: None,
            },
            2 => Self {
                bg1: Some(4),
                bg2: Some(4),
                bg3: None,
                bg4: None,
            },
            3 => Self {
                bg1: Some(8),
                bg2: Some(4),
                bg3: None,
                bg4: None,
            },
            _ => unimplemented!(),
        }
    }
}
