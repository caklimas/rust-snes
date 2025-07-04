pub mod registers;

pub use registers::Registers;

#[derive(Debug, Default)]
pub struct Cpu {
    registers: Registers,
}
