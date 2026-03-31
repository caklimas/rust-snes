use crate::apu::processor_status_word::ProcessorStatusWord;

#[derive(Default)]
pub struct Registers {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sp: u8,
    pub pc: u16,
    pub psw: ProcessorStatusWord,
}
