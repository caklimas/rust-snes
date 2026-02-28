use crate::memory::{
    addresses::{
        APU_REGISTERS_RANGE, NMI_STATUS_REGISTER, PPU_REGISTERS_RANGE, WRAM_RANGE, WRAM_START,
    },
    cartridge::Cartridge,
    memory_bus::MemoryBus,
    memory_region::MemoryRegion,
};

pub struct Bus {
    cartridge: Cartridge,
    nmi_status_value: u8,
    wram: MemoryRegion,
}

impl Bus {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            cartridge: Cartridge::new(data),
            nmi_status_value: 0x42,
            wram: MemoryRegion::new(vec![0; 131072], WRAM_START),
        }
    }

    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            NMI_STATUS_REGISTER => self.read_nmi_status(),
            addr if WRAM_RANGE.contains(&addr) => self.wram.read(&address),
            addr if PPU_REGISTERS_RANGE.contains(&addr) => {
                // PPU register access
                0
            }
            addr if APU_REGISTERS_RANGE.contains(&addr) => {
                // APU register access
                0
            }
            _ => self.cartridge.read(address),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            NMI_STATUS_REGISTER => self.write_nmi_status(value),
            addr if WRAM_RANGE.contains(&addr) => self.wram.write(&address, value),
            addr if PPU_REGISTERS_RANGE.contains(&addr) => {}
            addr if APU_REGISTERS_RANGE.contains(&addr) => {}
            _ => self.cartridge.write(address, value),
        }
    }

    fn read_nmi_status(&mut self) -> u8 {
        if self.nmi_status_value == 0x42 {
            self.nmi_status_value = 0xC2;
        } else if self.nmi_status_value == 0xC2 {
            self.nmi_status_value = 0x42;
        }

        self.nmi_status_value
    }

    fn write_nmi_status(&mut self, value: u8) {
        self.nmi_status_value = value;
    }
}

impl MemoryBus for Bus {
    fn read(&mut self, address: u32) -> u8 {
        self.read(address)
    }

    fn write(&mut self, address: u32, value: u8) {
        self.write(address, value)
    }
}
