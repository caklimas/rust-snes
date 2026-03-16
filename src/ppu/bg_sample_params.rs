use crate::ppu::bg_tilemap::BgTilemap;

pub struct BgSampleParams<'a> {
    pub enabled: bool,
    pub x: u16,
    pub y: u16,
    pub bg_tilemap: &'a BgTilemap,
    pub bg_horizontal_offset: u16,
    pub bg_vertical_offset: u16,
    pub char_base: u16,
    pub bpp_opt: Option<u8>,
    pub palette_base: u8,
    pub tile_size_16: bool,
}
