use std::fmt;

use crate::cpu::processor_status::ProcessorStatus;

pub struct Registers {
    pub a: u16,             // accumulator
    pub x: u16,             // index register
    pub y: u16,             // index register
    pub s: u16,             // stack pointer
    pub d: u16,             // direct page
    pub pc: u16,            // program counter
    pub pb: u8,             // program bank
    pub db: u8,             // data bank
    pub p: ProcessorStatus, // processor status/flags
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
            .field("a", &format_args!("${:04X}", self.a))
            .field("x", &format_args!("${:04X}", self.x))
            .field("y", &format_args!("${:04X}", self.y))
            .field("s", &format_args!("${:04X}", self.s))
            .field("d", &format_args!("${:04X}", self.d))
            .field("pc", &format_args!("${:02X}:{:04X}", self.pb, self.pc))
            .field("db", &format_args!("${:02X}", self.db))
            .field("p", &self.p)
            .finish()
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            s: 0x01FF,
            d: 0,
            pc: 0,
            pb: 0,
            db: 0,
            p: ProcessorStatus::from_bits_truncate(0x0034),
        }
    }
}
