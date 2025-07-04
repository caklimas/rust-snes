pub mod processor_status;
pub mod registers;

pub use registers::Registers;

use crate::{
    cpu::processor_status::ProcessorStatus,
    memory::{addresses, bus::Bus},
};

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
}

impl Cpu {
    pub fn step(&mut self, bus: &mut Bus) -> u8 {
        let data = bus.read(self.registers.pc as u32);
        match data {
            0xA9 => self.lda_immediate(bus),
            _ => (),
        }

        0
    }

    fn lda_immediate(&mut self, bus: &mut Bus) {
        let mut pc_increment = 2;
        let address = (self.registers.pc + 1) as u32;
        if self.registers.p.contains(ProcessorStatus::MEMORY_WIDTH) {
            let value = self.read_byte(bus, address);
            self.registers.a = (self.registers.a & 0xFF00) | value as u16;
            self.registers.p.set(ProcessorStatus::ZERO, value == 0);
            self.registers
                .p
                .set(ProcessorStatus::NEGATIVE, self.is_negative_u8(value));
        } else {
            let value = self.read_word(bus, address);
            self.registers.a = value;
            self.registers.p.set(ProcessorStatus::ZERO, value == 0);
            self.registers
                .p
                .set(ProcessorStatus::NEGATIVE, self.is_negative_u16(value));
            pc_increment += 1;
        }

        self.registers.pc += pc_increment;
    }

    fn read_word(&self, bus: &mut Bus, address: u32) -> u16 {
        let low = self.read_byte(bus, address);
        let high = self.read_byte(bus, address + 1);
        (high as u16) << 8 | (low as u16)
    }

    fn read_byte(&self, bus: &mut Bus, address: u32) -> u8 {
        bus.read(address)
    }

    fn is_negative_u8(&self, value: u8) -> bool {
        value & 0x80 != 0
    }

    fn is_negative_u16(&self, value: u16) -> bool {
        value & 0x8000 != 0
    }
}
