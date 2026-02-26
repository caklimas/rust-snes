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

    /// Read a 16-bit word (little-endian) from the given 24-bit address
    fn read_word(&mut self, address: u32) -> u16 {
        let lo = self.read(address);
        let hi = self.read((address.wrapping_add(1)) & 0x00FF_FFFF);
        u16::from_le_bytes([lo, hi])
    }

    /// Write a 16-bit word (little-endian) to the given 24-bit address
    fn write_word(&mut self, address: u32, value: u16) {
        self.write(address, value as u8);
        self.write((address.wrapping_add(1)) & 0x00FF_FFFF, (value >> 8) as u8);
    }
}
