#[derive(Debug)]
pub struct Registers {
    a: u16,  // accumulator
    x: u16,  // index register
    y: u16,  // index register
    s: u16,  // stack pointer
    d: u16,  // direct page,
    pc: u16, // progran counter
    pb: u8,  // program bank
    db: u8,  // data bank
    p: u8,   // processor status/flags
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
