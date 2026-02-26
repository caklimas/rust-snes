use rust_snes::memory::MemoryBus;
use std::collections::HashMap;

/// TestBus - A simple flat 24-bit address space for testing CPU opcodes
///
/// Unlike the real SNES bus with complex memory mapping, this provides
/// a straightforward 16MB address space (0x000000-0xFFFFFF) where any
/// address can store data. Perfect for running the SingleStepTests test suite.
pub struct TestBus {
    /// Sparse memory storage - only stores non-zero values
    memory: HashMap<u32, u8>,
}

impl TestBus {
    /// Creates a new empty TestBus
    pub fn new() -> Self {
        Self {
            memory: HashMap::new(),
        }
    }

    /// Initializes memory from a sparse array of (address, value) pairs
    /// This matches the format used in the test JSON files
    pub fn load_memory(&mut self, ram: &[(u32, u8)]) {
        for &(address, value) in ram {
            self.memory.insert(address, value);
        }
    }

    /// Reads a byte from memory
    /// Returns 0 for uninitialized addresses (matching test expectations)
    pub fn read(&mut self, address: u32) -> u8 {
        *self.memory.get(&address).unwrap_or(&0)
    }

    /// Writes a byte to memory
    pub fn write(&mut self, address: u32, value: u8) {
        if value == 0 {
            // Remove zero values to keep memory sparse
            self.memory.remove(&address);
        } else {
            self.memory.insert(address, value);
        }
    }

    /// Extracts memory values for specific addresses (for test verification)
    /// Returns a vector of (address, value) pairs for the given addresses
    pub fn extract_memory(&self, addresses: &[u32]) -> Vec<(u32, u8)> {
        addresses
            .iter()
            .map(|&addr| (addr, *self.memory.get(&addr).unwrap_or(&0)))
            .collect()
    }

    /// Clears all memory (useful for resetting between tests)
    pub fn clear(&mut self) {
        self.memory.clear();
    }
}

impl Default for TestBus {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryBus for TestBus {
    fn read(&mut self, address: u32) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u32, value: u8) {
        self.write(address, value)
    }
}
