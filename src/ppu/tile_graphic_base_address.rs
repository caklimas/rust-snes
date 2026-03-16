use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct TileGraphicBaseAddress(u8);

    pub first_bg_base, set_first_bg_base: 3, 0;
    pub second_bg_base, set_second_bg_base: 7, 4;
}

impl TileGraphicBaseAddress {
    pub fn first_vram_word_address(&self) -> u16 {
        ((self.first_bg_base() as u16) * 0x1000) & 0x7FFF
    }

    pub fn second_vram_word_address(&self) -> u16 {
        ((self.second_bg_base() as u16) * 0x1000) & 0x7FFF
    }
}
