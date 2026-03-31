use crate::apu::registers::Registers;

pub struct Spc700 {
    pub ipl_rom: [u8; 64],
    pub ram: [u8; 65536],
    pub registers: Registers,
}

impl Default for Spc700 {
    fn default() -> Self {
        Self {
            ipl_rom: [0; 64],
            ram: [0; 65536],
            registers: Default::default(),
        }
    }
}
