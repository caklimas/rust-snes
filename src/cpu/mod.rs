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
            IRQ_VECTOR_EMULATOR_HI, IRQ_VECTOR_EMULATOR_LO, IRQ_VECTOR_NATIVE_HI,
            IRQ_VECTOR_NATIVE_LO, NMI_VECTOR_EMULATOR_HI, NMI_VECTOR_EMULATOR_LO,
            NMI_VECTOR_NATIVE_HI, NMI_VECTOR_NATIVE_LO,
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
        self.enter_interrupt(
            bus,
            (NMI_VECTOR_NATIVE_LO, NMI_VECTOR_NATIVE_HI),
            (NMI_VECTOR_EMULATOR_LO, NMI_VECTOR_EMULATOR_HI),
        );
    }

    pub fn irq<B: MemoryBus>(&mut self, bus: &mut B) {
        self.enter_interrupt(
            bus,
            (IRQ_VECTOR_NATIVE_LO, IRQ_VECTOR_NATIVE_HI),
            (IRQ_VECTOR_EMULATOR_LO, IRQ_VECTOR_EMULATOR_HI),
        );
    }

    fn enter_interrupt<B: MemoryBus>(
        &mut self,
        bus: &mut B,
        vector_native: (u32, u32),
        vector_emulator: (u32, u32),
    ) {
        let pc_hi = ((self.registers.pc & 0xFF00) >> 8) as u8;
        let pc_lo = (self.registers.pc & 0x00FF) as u8;

        let (stack_mode, vector) = if self.emulation_mode {
            (opcodes::StackMode::EmuPage1, vector_emulator)
        } else {
            push_byte(self, bus, self.registers.pb, opcodes::StackMode::Linear16);
            (opcodes::StackMode::Linear16, vector_native)
        };

        push_byte(self, bus, pc_hi, stack_mode);
        push_byte(self, bus, pc_lo, stack_mode);
        push_byte(self, bus, self.registers.p.bits(), stack_mode);

        self.registers.p.set(ProcessorStatus::IRQ_DISABLE, true);
        self.registers.p.set(ProcessorStatus::DECIMAL, false);

        if !self.emulation_mode {
            self.registers.pb = 0;
        }

        let lo = bus.read(vector.0);
        let hi = bus.read(vector.1);
        self.registers.pc = u16::from_le_bytes([lo, hi]);

        self.waiting_for_interrupt = false;
    }
}
