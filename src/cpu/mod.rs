pub mod opcodes;
pub mod processor_status;
pub mod registers;

pub use registers::Registers;

use crate::{cpu::opcodes::execute_opcode, memory::bus::Bus};

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    emulation_mode: bool,
}

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let opcode = bus.read(self.registers.pc as u32);
        let cycles = execute_opcode(self, bus, opcode);

        cycles
    }
}
