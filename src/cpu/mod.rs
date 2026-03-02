pub mod opcodes;
pub mod processor_status;
pub mod registers;

pub use registers::Registers;

use crate::{cpu::opcodes::execute_opcode, memory::MemoryBus};

#[derive(Debug)]
pub struct Cpu {
    pub registers: Registers,
    pub emulation_mode: bool,
    pub waiting_for_interrupt: bool,
    pub stopped: bool,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            registers: Registers::default(),
            emulation_mode: true,
            waiting_for_interrupt: false,
            stopped: false,
        }
    }
}

impl Cpu {
    pub fn step<B: MemoryBus>(&mut self, bus: &mut B) -> u8 {
        // If CPU is stopped, do nothing
        if self.stopped {
            return 1;
        }

        // If CPU is waiting for interrupt, do nothing (interrupt handler will clear this flag)
        if self.waiting_for_interrupt {
            return 1;
        }

        let opcode_address = ((self.registers.pb as u32) << 16) | (self.registers.pc as u32);
        let opcode = bus.read(opcode_address);

        execute_opcode(self, bus, opcode)
    }
}
