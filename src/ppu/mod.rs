use crate::{
    memory::addresses::{
        BG1HOFS, BG1SC, BG1VOFS, BG2HOFS, BG2SC, BG2VOFS, BG3HOFS, BG3SC, BG3VOFS, BG4HOFS, BG4SC,
        BG4VOFS, BG12NBA, BG34NBA, BGMODE, CGADD, CGDATA, CGDATAREAD, INIDISP, OAMADD_HI,
        OAMADD_LO, OAMDATA, OAMDATAREAD, SETINI, TM, TS, VMADDH, VMADDL, VMAIN, VMDATAH, VMDATAL,
    },
    ppu::{
        bg_horizontal_offset::BgHorizontalOffset, bg_mode::BgMode, bg_tilemap::BgTilemap,
        bg_vertical_offset::BgVerticalOffset, cgram::Cgram, display::Display, oam::Oam,
        screen_designation::ScreenDesignation, screen_setting::ScreenSetting,
        tile_graphic_base_address::TileGraphicBaseAddress, tilemap_entry::TilemapEntry, vram::Vram,
    },
};

pub mod bg_horizontal_offset;
pub mod bg_mode;
pub mod bg_tilemap;
pub mod bg_vertical_offset;
pub mod cgram;
pub mod display;
pub mod oam;
pub mod screen_designation;
pub mod screen_setting;
pub mod tile_graphic_base_address;
pub mod tilemap_entry;
pub mod vmain;
pub mod vram;

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
    frame_buffer: [u16; 256 * 244],
    main_screen_designation: ScreenDesignation,
    oam: Oam,
    screen_setting: ScreenSetting,
    sub_screen_designation: ScreenDesignation,
    tile_graphic12: TileGraphicBaseAddress,
    tile_graphic34: TileGraphicBaseAddress,
    vram: Vram,
}

impl Ppu {
    pub fn render_scanline(&mut self, y: u16) {
        for x in 0u16..256 {
            let index = ((y * 256) + x) as usize;
            if self.display.forced_blank() {
                self.frame_buffer[index] = 0;
                continue;
            }

            let x_offset = x + self.bg_horizontal_offset.bg1_offset;
            let y_offset = y + self.bg_vertical_offset.bg1_offset;

            let tile_x = x_offset / 8;
            let tile_y = y_offset / 8;

            let pixel_x_within_tile = x_offset % 8;
            let pixel_y_within_tile = y_offset % 8;

            let tilemap_width = self.bg1.get_tilemap_width();
            let tilemap_height = self.bg1.get_tilemap_height();

            let entry_address = self.bg1.get_vram_word_address()
                + (tile_y % tilemap_height) * tilemap_width
                + (tile_x % tilemap_width);

            let tilemap_entry = TilemapEntry(self.vram.read_word(entry_address));
            let char_base = self.tile_graphic12.first_vram_word_address();
            let tile_base = char_base + tilemap_entry.tile_number() * 16;

            let bitplane_01_address = tile_base + pixel_y_within_tile * 2;
            let bitplane_23_address = tile_base + 8 + pixel_y_within_tile * 2;

            let bitplane_01 = self.vram.read_word(bitplane_01_address);
            let bitplane_23 = self.vram.read_word(bitplane_23_address);

            let bit = 7 - pixel_x_within_tile;
            let plane_0 = ((bitplane_01 & 0xFF) >> bit) & 0b1;
            let plane_1 = (((bitplane_01 & 0xFF00) >> 8) >> bit) & 0b1;
            let plane_2 = ((bitplane_23 & 0xFF) >> bit) & 0b1;
            let plane_3 = (((bitplane_23 & 0xFF00) >> 8) >> bit) & 0b1;

            let character_data = plane_0 | (plane_1 << 1) | (plane_2 << 2) | (plane_3 << 3);
        }
    }

    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            OAMADD_LO => 0,
            OAMADD_HI => 0,
            OAMDATA => 0,
            OAMDATAREAD => self.oam.read_oamdata(),
            CGADD => 0,
            CGDATA => 0,
            CGDATAREAD => self.cgram.read_cgdata(),
            _ => unimplemented!(),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            INIDISP => self.display.0 = value,
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
            _ => unimplemented!(),
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
            frame_buffer: [0; 256 * 244],
            main_screen_designation: Default::default(),
            oam: Default::default(),
            screen_setting: Default::default(),
            sub_screen_designation: Default::default(),
            tile_graphic12: Default::default(),
            tile_graphic34: Default::default(),
            vram: Default::default(),
        }
    }
}
