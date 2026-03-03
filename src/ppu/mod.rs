use crate::{
    memory::addresses::{
        CGADD, CGDATA, CGDATAREAD, INIDISP, OAMADD_HI, OAMADD_LO, OAMDATA, OAMDATAREAD, VMADDH,
        VMADDL, VMAIN, VMDATAH, VMDATAL,
    },
    ppu::{cgram::Cgram, display::Display, oam::Oam, vmain::Vmain, vram::Vram},
};

pub mod cgram;
pub mod display;
pub mod oam;
pub mod vmain;
pub mod vram;

#[derive(Default)]
pub struct Ppu {
    cgram: Cgram,
    display: Display,
    oam: Oam,
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
}
