use crate::ppu::bg_tilemap::BgTilemap;

pub struct BgSampleParams<'a> {
    pub main_enabled: bool,
    pub sub_enabled: bool,
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

impl<'a> BgSampleParams<'a> {
    pub fn new(
        main_enabled: bool,
        sub_enabled: bool,
        x: u16,
        y: u16,
        bg_tilemap: &'a BgTilemap,
        bg_horizontal_offset: u16,
        bg_vertical_offset: u16,
        char_base: u16,
        bpp_opt: Option<u8>,
        palette_base: u8,
        tile_size_16: bool,
        mosaic_enabled: bool,
        mosaic_size: u16,
    ) -> Self {
        let mut new_x = x;
        let mut new_y = y;

        if mosaic_enabled {
            new_x = new_x - (new_x % mosaic_size);
            new_y = new_y - ((new_y - 1) % mosaic_size);
        }

        Self {
            main_enabled,
            sub_enabled,
            x: new_x,
            y: new_y,
            bg_tilemap,
            bg_horizontal_offset,
            bg_vertical_offset,
            char_base,
            bpp_opt,
            palette_base,
            tile_size_16,
        }
    }
}
