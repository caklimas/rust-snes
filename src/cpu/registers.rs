#[derive(Debug)]
pub struct Registers {
    pub a: u16,  // accumulator
    pub x: u16,  // index register
    pub y: u16,  // index register
    pub s: u16,  // stack pointer
    pub d: u16,  // direct page,
    pub pc: u16, // progran counter
    pub pb: u8,  // program bank
    pub db: u8,  // data bank
    pub p: u8,   // processor status/flags
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
            p: 0x0034,
        }
    }
}
