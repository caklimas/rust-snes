use crate::memory::{addresses::CARTRIDGE_ROM_START, memory_region::MemoryRegion};

pub struct Cartridge {
    memory: MemoryRegion,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        Cartridge {
            memory: MemoryRegion::new(data, CARTRIDGE_ROM_START),
        }
    }

    pub fn read(&self, address: &u32) -> u8 {
        self.memory.read(address)
    }
}
