use crate::apu::processor_status_word::ProcessorStatusWord;

pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub psw: ProcessorStatusWord,
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
