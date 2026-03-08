use crate::{
    memory::addresses::{
        BG1HOFS, BG1SC, BG1VOFS, BG2HOFS, BG2SC, BG2VOFS, BG3HOFS, BG3SC, BG3VOFS, BG4HOFS, BG4SC,
        BG4VOFS, BG12NBA, BG34NBA, BGMODE, CGADD, CGDATA, CGDATAREAD, INIDISP, OAMADD_HI,
        OAMADD_LO, OAMDATA, OAMDATAREAD, OBSEL, SETINI, TM, TS, VMADDH, VMADDL, VMAIN, VMDATAH,
        VMDATAL,
    },
    ppu::{
        bg_horizontal_offset::BgHorizontalOffset, bg_mode::BgMode, bg_tilemap::BgTilemap,
        bg_vertical_offset::BgVerticalOffset, bpp_settings::BppSettings, cgram::Cgram,
        display::Display, oam::Oam, obsel::Obsel, palette_base::PaletteBase,
        screen_designation::ScreenDesignation, screen_setting::ScreenSetting,
        tile_graphic_base_address::TileGraphicBaseAddress, tilemap_entry::TilemapEntry, vram::Vram,
    },
};

pub mod bg_horizontal_offset;
pub mod bg_mode;
pub mod bg_tilemap;
pub mod bg_vertical_offset;
pub mod bpp_settings;
pub mod cgram;
pub mod display;
pub mod high_table_sprite;
pub mod low_table_sprite;
pub mod oam;
pub mod obsel;
pub mod packed_attributes;
pub mod palette_base;
pub mod rgb;
pub mod screen_designation;
pub mod screen_setting;
pub mod tile_graphic_base_address;
pub mod tilemap_entry;
pub mod vmain;
pub mod vram;

pub const SCREEN_WIDTH: u16 = 256;
pub const SCREEN_HEIGHT: u16 = 224;

pub struct Ppu {
    bg1: BgTilemap,
    bg2: BgTilemap,
    bg3: BgTilemap,
    bg4: BgTilemap,
    bg_horizontal_offset: BgHorizontalOffset,
    bg_vertical_offset: BgVerticalOffset,
    bg_mode: BgMode,
    bg_old: u8,
    cgram: Cgram,
    display: Display,
    frame_buffer: [u16; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize],
    main_screen_designation: ScreenDesignation,
    oam: Oam,
    obsel: Obsel,
    screen_setting: ScreenSetting,
    sub_screen_designation: ScreenDesignation,
    tile_graphic12: TileGraphicBaseAddress,
    tile_graphic34: TileGraphicBaseAddress,
    pub vram: Vram,
}

impl Ppu {
    pub fn frame_buffer(&self) -> &[u16] {
        &self.frame_buffer
    }

    pub fn render_scanline(&mut self, y: u16) {
        let bpp_settings = BppSettings::new(&self.bg_mode);
        let palette_base = PaletteBase::new(&self.bg_mode);

        for x in 0u16..SCREEN_WIDTH {
            let index = ((y * SCREEN_WIDTH) + x) as usize;
            if self.display.forced_blank() {
                self.frame_buffer[index] = 0;
                continue;
            }

            let bg1_sample = self.bg_sample(
                self.main_screen_designation.bg1_enable(),
                x,
                y,
                &self.bg1,
                self.bg_horizontal_offset.bg1_offset,
                self.bg_vertical_offset.bg1_offset,
                self.tile_graphic12.first_vram_word_address(),
                bpp_settings.bg1,
                palette_base.bg1,
            );

            let bg2_sample = self.bg_sample(
                self.main_screen_designation.bg2_enable(),
                x,
                y,
                &self.bg2,
                self.bg_horizontal_offset.bg2_offset,
                self.bg_vertical_offset.bg2_offset,
                self.tile_graphic12.second_vram_word_address(),
                bpp_settings.bg2,
                palette_base.bg2,
            );

            let bg3_sample = self.bg_sample(
                self.main_screen_designation.bg3_enable(),
                x,
                y,
                &self.bg3,
                self.bg_horizontal_offset.bg3_offset,
                self.bg_vertical_offset.bg3_offset,
                self.tile_graphic34.first_vram_word_address(),
                bpp_settings.bg3,
                palette_base.bg3,
            );

            let obj_sample = self.obj_sample(x, y);
            let sample = if let Some((cgram, 3)) = obj_sample {
                Some(cgram)
            } else if let Some((cgram, true)) = bg1_sample {
                Some(cgram)
            } else if let Some((cgram, true)) = bg2_sample {
                Some(cgram)
            } else if let Some((cgram, 2)) = obj_sample {
                Some(cgram)
            } else if let Some((cgram, false)) = bg1_sample {
                Some(cgram)
            } else if let Some((cgram, false)) = bg2_sample {
                Some(cgram)
            } else if let Some((cgram, 1)) = obj_sample {
                Some(cgram)
            } else if let Some((cgram, true)) = bg3_sample {
                Some(cgram)
            } else if let Some((cgram, 0)) = obj_sample {
                Some(cgram)
            } else if let Some((cgram, false)) = bg3_sample {
                Some(cgram)
            } else {
                None
            };

            let color = match sample {
                Some(cgram_index) => self.cgram.read_color(cgram_index),
                None => self.cgram.read_color(0),
            };

            self.frame_buffer[index] = color;
        }
    }

    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            OAMADD_LO => 0,
            OAMADD_HI => 0,
            OAMDATA => 0,
            BGMODE => self.bg_mode.0,
            BG1SC => self.bg1.0,
            BG2SC => self.bg2.0,
            BG3SC => self.bg3.0,
            BG4SC => self.bg4.0,
            BG12NBA => self.tile_graphic12.0,
            BG34NBA => self.tile_graphic34.0,
            TM => self.main_screen_designation.0,
            TS => self.sub_screen_designation.0,
            OAMDATAREAD => self.oam.read_oamdata(),
            CGADD => 0,
            CGDATA => 0,
            CGDATAREAD => self.cgram.read_cgdata(),
            _ => {
                eprintln!("Unhandled PPU read: {:#06X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            INIDISP => self.display.0 = value,
            OBSEL => self.obsel.0 = value,
            OAMADD_LO => self.oam.set_oamadd(value, true),
            OAMADD_HI => self.oam.set_oamadd(value, false),
            OAMDATA => self.oam.write_oamdata(value),
            BGMODE => self.bg_mode.0 = value,
            BG1SC => self.bg1.0 = value,
            BG2SC => self.bg2.0 = value,
            BG3SC => self.bg3.0 = value,
            BG4SC => self.bg4.0 = value,
            BG12NBA => self.tile_graphic12.0 = value,
            BG34NBA => self.tile_graphic34.0 = value,
            BG1HOFS => self.set_horizontal_offset(1, value),
            BG1VOFS => self.set_vertical_offset(1, value),
            BG2HOFS => self.set_horizontal_offset(2, value),
            BG2VOFS => self.set_vertical_offset(2, value),
            BG3HOFS => self.set_horizontal_offset(3, value),
            BG3VOFS => self.set_vertical_offset(3, value),
            BG4HOFS => self.set_horizontal_offset(4, value),
            BG4VOFS => self.set_vertical_offset(4, value),
            TM => self.main_screen_designation.0 = value,
            TS => self.sub_screen_designation.0 = value,
            SETINI => self.screen_setting.0 = value,
            OAMDATAREAD => {}
            VMAIN => self.vram.vmain.0 = value,
            VMADDL => self.vram.set_address_lo(value),
            VMADDH => self.vram.set_address_hi(value),
            VMDATAL => self.vram.write_data_lo(value),
            VMDATAH => self.vram.write_data_hi(value),
            CGADD => self.cgram.write_cgadd(value),
            CGDATA => self.cgram.write_cgdata(value),
            CGDATAREAD => {}
            _ => eprintln!("Unhandled PPU write: {:#06X} = {:#04X}", address, value),
        }
    }

    fn set_horizontal_offset(&mut self, number: u8, value: u8) {
        let offset = ((value as u16) << 8) | ((self.bg_old as u16) & !7);
        match number {
            1 => {
                self.bg_horizontal_offset.bg1_offset =
                    offset | ((self.bg_horizontal_offset.bg1_offset >> 8) & 7)
            }
            2 => {
                self.bg_horizontal_offset.bg2_offset =
                    offset | ((self.bg_horizontal_offset.bg2_offset >> 8) & 7)
            }
            3 => {
                self.bg_horizontal_offset.bg3_offset =
                    offset | ((self.bg_horizontal_offset.bg3_offset >> 8) & 7)
            }
            4 => {
                self.bg_horizontal_offset.bg4_offset =
                    offset | ((self.bg_horizontal_offset.bg4_offset >> 8) & 7)
            }
            _ => unimplemented!(),
        }

        self.bg_old = value;
    }

    fn set_vertical_offset(&mut self, number: u8, value: u8) {
        let offset = ((value as u16) << 8) | (self.bg_old as u16);
        match number {
            1 => self.bg_vertical_offset.bg1_offset = offset,
            2 => self.bg_vertical_offset.bg2_offset = offset,
            3 => self.bg_vertical_offset.bg3_offset = offset,
            4 => self.bg_vertical_offset.bg4_offset = offset,
            _ => unimplemented!(),
        }

        self.bg_old = value;
    }

    fn bg_sample(
        &self,
        enabled: bool,
        x: u16,
        y: u16,
        bg_tilemap: &BgTilemap,
        bg_horizontal_offset: u16,
        bg_vertical_offset: u16,
        char_base: u16,
        bpp_opt: Option<u8>,
        palette_base: u8,
    ) -> Option<(u8, bool)> {
        if !enabled {
            return None;
        }

        let bpp = bpp_opt?;
        let x_offset = bg_horizontal_offset.wrapping_add(x);
        let y_offset = bg_vertical_offset.wrapping_add(y);

        let tile_x = x_offset / 8;
        let tile_y = y_offset / 8;

        let tilemap_width = bg_tilemap.get_tilemap_width();
        let tilemap_height = bg_tilemap.get_tilemap_height();

        let entry_address = bg_tilemap.get_vram_word_address()
            + (tile_y % tilemap_height) * tilemap_width
            + (tile_x % tilemap_width);

        let tilemap_entry = TilemapEntry(self.vram.read_word(entry_address));
        let tile_base_multiplier = if bpp == 4 { 16 } else { 8 };
        let tile_base = char_base + tilemap_entry.tile_number() * tile_base_multiplier;

        let pixel_x_within_tile = x_offset % 8;
        let mut pixel_y_within_tile = y_offset % 8;

        if tilemap_entry.y_flip() {
            pixel_y_within_tile = 7 - pixel_y_within_tile;
        }

        let bitplane_01_address = tile_base + pixel_y_within_tile;
        let bitplane_01 = self.vram.read_word(bitplane_01_address);

        let bit = if tilemap_entry.x_flip() {
            pixel_x_within_tile
        } else {
            7 - pixel_x_within_tile
        };
        let plane_0 = ((bitplane_01 & 0xFF) >> bit) & 0b1;
        let plane_1 = (((bitplane_01 & 0xFF00) >> 8) >> bit) & 0b1;

        let character_data = if bpp == 4 {
            let bitplane_23_address = tile_base + 8 + pixel_y_within_tile;
            let bitplane_23 = self.vram.read_word(bitplane_23_address);
            let plane_2 = ((bitplane_23 & 0xFF) >> bit) & 0b1;
            let plane_3 = (((bitplane_23 & 0xFF00) >> 8) >> bit) & 0b1;

            plane_0 | (plane_1 << 1) | (plane_2 << 2) | (plane_3 << 3)
        } else {
            plane_0 | (plane_1 << 1)
        };

        if character_data == 0 {
            None
        } else {
            let palette_multiplier = if bpp == 4 { 16 } else { 4 };
            Some((
                ((palette_base as u16)
                    + tilemap_entry.palette_number() * palette_multiplier
                    + character_data) as u8,
                tilemap_entry.tile_priority(),
            ))
        }
    }

    fn obj_sample(&self, x: u16, y: u16) -> Option<(u8, u8)> {
        for i in 0..128 {
            let (low, high) = self.oam.get_sprite(i);
            let x_full = low.x as i16 | ((high.x_position_bit_8() as i16) << 8);
            let x_signed = if x_full >= 256 { x_full - 512 } else { x_full };
            let y_signed = low.y as i16;
            let sprite_size = self.obsel.get_object_size(high.size());
            let bounds_check = (x as i16) >= x_signed
                && (x as i16) < x_signed + sprite_size as i16
                && (y as i16) >= y_signed
                && (y as i16) < y_signed + sprite_size as i16;

            if !bounds_check {
                continue;
            }
            let tile_col = (x as i16) - x_signed;
            let tile_row = (y as i16) - y_signed;

            let sub_tile_col = (tile_col / 8) as u8;
            let sub_tile_row = (tile_row / 8) as u8;

            let lower_nibble = ((low.tile_number & 0x0F) + sub_tile_col) & 0x0F;
            let upper_nibble = (low.tile_number & 0xF0) + (sub_tile_row * 0x10);

            let tile_number = upper_nibble | lower_nibble;
            let base_word_address = if low.packed_attributes.name_table() {
                ((self.obsel.name_base() as u16).wrapping_mul(0x2000))
                    .wrapping_add((self.obsel.name_select() as u16 + 1).wrapping_mul(0x1000))
            } else {
                (self.obsel.name_base() as u16).wrapping_mul(0x2000)
            };

            let tile_word_address =
                base_word_address.wrapping_add((tile_number as u16).wrapping_mul(16));
            let pixel_x_within_tile = (tile_col % 8) as u16;
            let mut pixel_y_within_tile = (tile_row % 8) as u16;
            if low.packed_attributes.y_flip() {
                pixel_y_within_tile = 7 - pixel_y_within_tile;
            }

            let bitplane_01_address = tile_word_address + pixel_y_within_tile;
            let bitplane_01 = self.vram.read_word(bitplane_01_address);

            let bit = if low.packed_attributes.x_flip() {
                pixel_x_within_tile
            } else {
                7 - pixel_x_within_tile
            };
            let plane_0 = ((bitplane_01 & 0xFF) >> bit) & 0b1;
            let plane_1 = (((bitplane_01 & 0xFF00) >> 8) >> bit) & 0b1;

            let bitplane_23_address = tile_word_address + 8 + pixel_y_within_tile;
            let bitplane_23 = self.vram.read_word(bitplane_23_address);
            let plane_2 = ((bitplane_23 & 0xFF) >> bit) & 0b1;
            let plane_3 = (((bitplane_23 & 0xFF00) >> 8) >> bit) & 0b1;

            let character_data = plane_0 | (plane_1 << 1) | (plane_2 << 2) | (plane_3 << 3);

            if character_data == 0 {
                continue;
            } else {
                return Some((
                    (128 + low.packed_attributes.palette() as u16 * 16 + character_data) as u8,
                    low.packed_attributes.priority(),
                ));
            }
        }

        None
    }
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            bg1: Default::default(),
            bg2: Default::default(),
            bg3: Default::default(),
            bg4: Default::default(),
            bg_horizontal_offset: Default::default(),
            bg_vertical_offset: Default::default(),
            bg_mode: Default::default(),
            bg_old: Default::default(),
            cgram: Default::default(),
            display: Default::default(),
            frame_buffer: [0; (SCREEN_HEIGHT * SCREEN_WIDTH) as usize],
            main_screen_designation: Default::default(),
            oam: Default::default(),
            obsel: Default::default(),
            screen_setting: Default::default(),
            sub_screen_designation: Default::default(),
            tile_graphic12: Default::default(),
            tile_graphic34: Default::default(),
            vram: Default::default(),
        }
    }
}
