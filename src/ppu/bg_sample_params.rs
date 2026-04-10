use crate::ppu::{bg_tilemap::BgTilemap, mosaic_config::MosaicConfig};

pub struct BgLayerConfig<'a> {
    pub bg_tilemap: &'a BgTilemap,
    pub horizontal_offset: u16,
    pub vertical_offset: u16,
    pub char_base: u16,
    pub bpp_opt: Option<u8>,
    pub palette_base: u8,
    pub tile_size_16: bool,
}

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
        layer: &BgLayerConfig<'a>,
        mosaic: &MosaicConfig,
    ) -> Self {
        let mut new_x = x;
        let mut new_y = y;

        if mosaic.enabled {
            new_x = new_x - (new_x % mosaic.size);
            new_y = new_y - ((new_y - 1) % mosaic.size);
        }

        Self {
            main_enabled,
            sub_enabled,
            x: new_x,
            y: new_y,
            bg_tilemap: layer.bg_tilemap,
            bg_horizontal_offset: layer.horizontal_offset,
            bg_vertical_offset: layer.vertical_offset,
            char_base: layer.char_base,
            bpp_opt: layer.bpp_opt,
            palette_base: layer.palette_base,
            tile_size_16: layer.tile_size_16,
        }
    }
}
