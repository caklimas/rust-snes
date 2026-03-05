pub mod opcodes;
pub mod processor_status;
pub mod registers;

pub use registers::Registers;

use crate::{
    cpu::{
        opcodes::{execute_opcode, push_byte},
        processor_status::ProcessorStatus,
    },
    memory::{
        MemoryBus,
        addresses::{
            NMI_VECTOR_EMULATOR_HI, NMI_VECTOR_EMULATOR_LO, NMI_VECTOR_NATIVE_HI,
            NMI_VECTOR_NATIVE_LO,
        },
    },
};

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

    pub fn nmi<B: MemoryBus>(&mut self, bus: &mut B) {
        if self.emulation_mode {
            let pc_hi = ((self.registers.pc & 0xFF00) >> 8) as u8;
            let pc_lo = (self.registers.pc & 0x00FF) as u8;

            push_byte(self, bus, pc_hi, opcodes::StackMode::EmuPage1);
            push_byte(self, bus, pc_lo, opcodes::StackMode::EmuPage1);
            push_byte(
                self,
                bus,
                self.registers.p.bits(),
                opcodes::StackMode::EmuPage1,
            );

            self.registers.p.set(ProcessorStatus::IRQ_DISABLE, true);
            self.registers.p.set(ProcessorStatus::DECIMAL, false);

            let lo = bus.read(NMI_VECTOR_EMULATOR_LO);
            let hi = bus.read(NMI_VECTOR_EMULATOR_HI);

            self.registers.pc = u16::from_le_bytes([lo, hi]);

            self.waiting_for_interrupt = false;
        } else {
            push_byte(self, bus, self.registers.pb, opcodes::StackMode::Linear16);

            let pc_hi = ((self.registers.pc & 0xFF00) >> 8) as u8;
            let pc_lo = (self.registers.pc & 0x00FF) as u8;

            push_byte(self, bus, pc_hi, opcodes::StackMode::Linear16);
            push_byte(self, bus, pc_lo, opcodes::StackMode::Linear16);
            push_byte(
                self,
                bus,
                self.registers.p.bits(),
                opcodes::StackMode::Linear16,
            );

            self.registers.p.set(ProcessorStatus::IRQ_DISABLE, true);
            self.registers.p.set(ProcessorStatus::DECIMAL, false);
            self.registers.pb = 0;

            let lo = bus.read(NMI_VECTOR_NATIVE_LO);
            let hi = bus.read(NMI_VECTOR_NATIVE_HI);

            self.registers.pc = u16::from_le_bytes([lo, hi]);

            self.waiting_for_interrupt = false;
        }
    }
}
