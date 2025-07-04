use crate::memory::{
    addresses::{
        APU_REGISTERS_RANGE, CARTRIDGE_ROM_RANGE, NMI_STATUS_REGISTER, PPU_REGISTERS_RANGE,
        WRAM_RANGE, WRAM_START,
    },
    memory_region::MemoryRegion,
};

pub struct Bus {
    nmi_status_value: u8,
    wram: MemoryRegion,
}

impl Bus {
    pub fn read(&self, address: u32) -> u8 {
        match address {
            NMI_STATUS_REGISTER => {}
            addr if WRAM_RANGE.contains(&addr) => {
                // WRAM access
            }
            addr if PPU_REGISTERS_RANGE.contains(&addr) => {
                // PPU register access
            }
            addr if APU_REGISTERS_RANGE.contains(&addr) => {
                // APU register access
            }
            addr if CARTRIDGE_ROM_RANGE.contains(&addr) => {
                // Cartridge ROM access
            }
            _ => {
                // Unknown/unmapped address
            }
        }

        0
    }
}

impl Default for Bus {
    fn default() -> Self {
        Self {
            nmi_status_value: 0x42,
            wram: MemoryRegion::new(vec![0; 131072], WRAM_START),
        }
    }
}
