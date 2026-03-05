use std::ops::RangeInclusive;

use crate::{
    memory::{
        addresses::{
            APU_REGISTERS_RANGE, DMA_REGISTERS_RANGE, DMA_REGISTERS_START, MDMAEN,
            NMI_STATUS_REGISTER, PPU_REGISTERS_RANGE, PPU_REGISTERS_START, UNUSED_IO_GAP_RANGE,
            UNUSED_UPPER_GAP_RANGE, WMADDH, WMADDL, WMADDM, WMDATA, WRAM_MIRROR_OFFSET_END,
            WRAM_MIRROR_OFFSET_START, WRAM_RANGE, WRAM_START,
        },
        cartridge::Cartridge,
        dma_channel::DmaChannel,
        memory_bus::MemoryBus,
        memory_region::MemoryRegion,
        nmi_status::NmiStatus,
        wram_access_address::WramAccessAddress,
    },
    ppu::Ppu,
};

const SYSTEM_MIRROR_BANK_RANGE: RangeInclusive<u8> = 0x80..=0xBF;
const SYSTEM_MIRROR_MASK: u32 = 0x7FFFFF;
const WRAM_ACCESS_MASK: u32 = 0x1FFFF;

pub struct Bus {
    pub ppu: Ppu,

    cartridge: Cartridge,
    dma_channels: [DmaChannel; 8],
    nmi_status: NmiStatus,
    wram: MemoryRegion,
    wram_access_address: WramAccessAddress,
}

impl Bus {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            cartridge: Cartridge::new(data),
            dma_channels: [Default::default(); 8],
            nmi_status: Default::default(),
            ppu: Default::default(),
            wram: MemoryRegion::new(vec![0; 131072], WRAM_START),
            wram_access_address: WramAccessAddress::default(),
        }
    }

    pub fn read(&mut self, address: u32) -> u8 {
        let normalized_address = Self::normalize_address(address);
        match normalized_address {
            addr if UNUSED_IO_GAP_RANGE.contains(&addr) => 0,
            WMDATA => {
                let value = self.wram.read(&self.get_wram_access_address());
                self.increment_wram_access_address();
                value
            }
            addr if UNUSED_UPPER_GAP_RANGE.contains(&addr) => 0,
            NMI_STATUS_REGISTER => {
                let value = self.nmi_status.0;
                self.nmi_status.set_nmi_flag(false);
                value
            }
            addr if WRAM_RANGE.contains(&addr) => self.wram.read(&addr),
            addr if Self::is_wram_mirror(addr) => {
                let wram_addr = WRAM_START + (addr & 0xFFFF);
                self.wram.read(&wram_addr)
            }
            addr if PPU_REGISTERS_RANGE.contains(&addr) => self.ppu.read(addr),
            addr if APU_REGISTERS_RANGE.contains(&addr) => {
                // APU register access
                0
            }
            _ => self.cartridge.read(address),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        let normalized_address = Self::normalize_address(address);
        match normalized_address {
            addr if UNUSED_IO_GAP_RANGE.contains(&addr) => {}
            WMDATA => {
                self.wram.write(&self.get_wram_access_address(), value);
                self.increment_wram_access_address();
            }
            WMADDL => self.wram_access_address.set_wmaddl(value as u32),
            WMADDM => self.wram_access_address.set_wmaddm(value as u32),
            WMADDH => self.wram_access_address.set_wmaddh(value & 0b1 == 1),
            addr if UNUSED_UPPER_GAP_RANGE.contains(&addr) => {}
            MDMAEN => {
                for i in 0u8..=7 {
                    let channel = (value >> i) & 0x1 == 1;
                    let mut incremented = false;
                    if channel {
                        let channel = self.dma_channels[i as usize];
                        let source = (channel.a1b as u32) << 16 | (channel.a1t as u32);
                        let destination = PPU_REGISTERS_START | (channel.bbad as u32);
                        let dmap_mode = channel.dmap & 0x07;
                        let transfer_direction = channel.dmap >> 7;
                        let das = if channel.das == 0 {
                            65536
                        } else {
                            channel.das as u32
                        };

                        for i in 0u32..das {
                            let destination = if dmap_mode == 1 && incremented {
                                destination + 1
                            } else {
                                destination
                            };

                            incremented = !incremented;

                            if transfer_direction == 0 {
                                let value = self.read(source + i);
                                self.write(destination, value);
                            } else {
                                let value = self.read(destination);
                                self.write(source + i, value);
                            }
                        }
                    }
                }
            }
            addr if DMA_REGISTERS_RANGE.contains(&addr) => {
                let offset = addr - DMA_REGISTERS_START;
                let upper_nibble = offset >> 4;
                self.dma_channels[upper_nibble as usize].set_register((offset & 0xF) as u8, value);
            }
            addr if WRAM_RANGE.contains(&addr) => self.wram.write(&addr, value),
            addr if Self::is_wram_mirror(addr) => {
                let wram_addr = WRAM_START + (addr & 0xFFFF);
                self.wram.write(&wram_addr, value)
            }
            addr if PPU_REGISTERS_RANGE.contains(&addr) => {
                self.ppu.write(normalized_address, value)
            }
            addr if APU_REGISTERS_RANGE.contains(&addr) => {}
            _ => self.cartridge.write(address, value),
        }
    }

    fn is_wram_mirror(address: u32) -> bool {
        let bank = (address >> 16) as u8;
        let offset = (address & 0xFFFF) as u16;
        matches!(bank, 0x00..=0x3F)
            && (WRAM_MIRROR_OFFSET_START..=WRAM_MIRROR_OFFSET_END).contains(&offset)
    }

    fn normalize_address(address: u32) -> u32 {
        if Self::is_mirror_bank(address) {
            address & SYSTEM_MIRROR_MASK
        } else {
            address
        }
    }

    fn is_mirror_bank(address: u32) -> bool {
        let bank_byte = (address >> 16) as u8;
        SYSTEM_MIRROR_BANK_RANGE.contains(&bank_byte)
    }

    fn get_wram_access_address(&self) -> u32 {
        self.wram_access_address.0 + WRAM_START
    }

    fn increment_wram_access_address(&mut self) {
        self.wram_access_address.0 = (self.wram_access_address.0 + 1) & WRAM_ACCESS_MASK
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
