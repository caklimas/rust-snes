/// MemoryBus trait - Abstraction over different bus implementations
///
/// This trait allows the CPU to work with different bus implementations:
/// - `Bus`: The real SNES memory bus with proper hardware mapping
/// - `TestBus`: A simple flat address space for running tests
///
/// Any type that can read and write bytes at 24-bit addresses can implement this trait.
pub trait MemoryBus {
    /// Read a byte from the given 24-bit address
    fn read(&mut self, address: u32) -> u8;

    /// Write a byte to the given 24-bit address
    fn write(&mut self, address: u32, value: u8);
}
