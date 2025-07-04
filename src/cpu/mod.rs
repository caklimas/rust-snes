pub mod processor_status;
pub mod registers;

pub use registers::Registers;

use crate::memory::bus::Bus;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn step(&self, bus: &mut Bus) -> u8 {
        let data = bus.read(self.registers.pc as u32);

        0
    }
}
