pub mod opcodes;
pub mod processor_status;
pub mod registers;

pub use registers::Registers;

use crate::{cpu::opcodes::execute_opcode, memory::bus::Bus};

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
    emulation_mode: bool,
    waiting_for_interrupt: bool,
    stopped: bool,
}

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        // If CPU is stopped, do nothing
        if self.stopped {
            return 1;
        }

        // If CPU is waiting for interrupt, do nothing (interrupt handler will clear this flag)
        if self.waiting_for_interrupt {
            return 1;
        }

        let opcode = bus.read(self.registers.pc as u32);

        execute_opcode(self, bus, opcode)
    }
}
