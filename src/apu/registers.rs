use std::fmt;

use crate::apu::processor_status_word::ProcessorStatusWord;

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub psw: ProcessorStatusWord,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Registers")
            .field("a", &format_args!("${:02X}", self.a))
            .field("x", &format_args!("${:02X}", self.x))
            .field("y", &format_args!("${:02X}", self.y))
            .field("sp", &format_args!("${:02X}", self.sp))
            .field("pc", &format_args!("${:04X}", self.pc))
            .field("psw", &self.psw)
            .finish()
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: Default::default(),
            x: Default::default(),
            y: Default::default(),
            sp: Default::default(),
            pc: 0xFFC0,
            psw: Default::default(),
        }
    }
}
