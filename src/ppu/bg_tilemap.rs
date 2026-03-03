use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct BgTilemap(u8);

    pub mirror_size, set_mirror_size: 1, 0;
    pub base_address, set_base_address: 7, 2;
}

impl BgTilemap {
    pub fn get_vram_word_address(&self) -> u16 {
        ((self.base_address() as u16) * 0x400) & 0x7FFF
    }
}
