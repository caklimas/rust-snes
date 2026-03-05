use crate::{
    cpu::Cpu,
    memory::{
        addresses::{RESET_VECTOR_HI, RESET_VECTOR_LO},
        bus::Bus,
    },
};

pub struct SuperNintendo {
    bus: Bus,
    cpu: Cpu,
    current_scanline: u16,
    cycles: u32,
}

impl SuperNintendo {
    pub fn new(data: Vec<u8>) -> Self {
        let mut bus = Bus::new(data);
        let mut cpu = Cpu::default();

        let lo = bus.read(RESET_VECTOR_LO);
        let hi = bus.read(RESET_VECTOR_HI);
        cpu.registers.pc = u16::from_le_bytes([lo, hi]);

        Self {
            bus,
            cpu,
            current_scanline: 0,
            cycles: 0,
        }
    }

    pub fn step(&mut self) {
        self.cycles += self.cpu.step(&mut self.bus) as u32;

        if self.cycles >= 227 {
            self.cycles -= 227;
            self.bus.ppu.render_scanline(self.current_scanline);

            self.current_scanline = (self.current_scanline + 1) % 224;
        }
    }
}
