use crate::{
    memory::addresses::{
        BG1HOFS, BG1SC, BG1VOFS, BG2HOFS, BG2SC, BG2VOFS, BG3HOFS, BG3SC, BG3VOFS, BG4HOFS, BG4SC,
        BG4VOFS, BG12NBA, BG34NBA, BGMODE, CGADD, CGDATA, CGDATAREAD, INIDISP, OAMADD_HI,
        OAMADD_LO, OAMDATA, OAMDATAREAD, VMADDH, VMADDL, VMAIN, VMDATAH, VMDATAL,
    },
    ppu::{
        bg_horizontal_offset::BgHorizontalOffset, bg_mode::BgMode, bg_tilemap::BgTilemap,
        bg_vertical_offset::BgVerticalOffset, cgram::Cgram, display::Display, oam::Oam,
        tile_graphic_base_address::TileGraphicBaseAddress, vmain::Vmain, vram::Vram,
    },
};

pub mod bg_horizontal_offset;
pub mod bg_mode;
pub mod bg_tilemap;
pub mod bg_vertical_offset;
pub mod cgram;
pub mod display;
pub mod oam;
pub mod tile_graphic_base_address;
pub mod vmain;
pub mod vram;

#[derive(Default)]
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
    oam: Oam,
    tile_graphic12: TileGraphicBaseAddress,
    tile_graphic34: TileGraphicBaseAddress,
    vram: Vram,
}

impl Ppu {
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
            INIDISP => self.display = Display(value),
            OAMADD_LO => self.oam.set_oamadd(value, true),
            OAMADD_HI => self.oam.set_oamadd(value, false),
            OAMDATA => self.oam.write_oamdata(value),
            BGMODE => self.bg_mode = BgMode(value),
            BG1SC => self.bg1 = BgTilemap(value),
            BG2SC => self.bg2 = BgTilemap(value),
            BG3SC => self.bg3 = BgTilemap(value),
            BG4SC => self.bg4 = BgTilemap(value),
            BG12NBA => self.tile_graphic12 = TileGraphicBaseAddress(value),
            BG34NBA => self.tile_graphic34 = TileGraphicBaseAddress(value),
            BG1HOFS => self.set_horizontal_offset(1, value),
            BG1VOFS => self.set_vertical_offset(1, value),
            BG2HOFS => self.set_horizontal_offset(2, value),
            BG2VOFS => self.set_vertical_offset(2, value),
            BG3HOFS => self.set_horizontal_offset(3, value),
            BG3VOFS => self.set_vertical_offset(3, value),
            BG4HOFS => self.set_horizontal_offset(4, value),
            BG4VOFS => self.set_vertical_offset(4, value),
            OAMDATAREAD => {}
            VMAIN => self.vram.vmain = Vmain(value),
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
