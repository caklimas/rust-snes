use crate::memory::{
    addresses::{HIROM_SRAM_START, LOROM_SRAM_START},
    cartridge_header::{CartridgeHeader, MappingMode},
    memory_region::MemoryRegion,
};

pub struct Cartridge {
    pub header: CartridgeHeader,
    pub sram: MemoryRegion,
    data: Vec<u8>,
}

impl Cartridge {
    pub fn new(data: Vec<u8>) -> Self {
        let data = Self::strip_smc_header(data);
        let header = CartridgeHeader::new(&data);
        let sram_base = match header.mapping_mode {
            MappingMode::HiRom => HIROM_SRAM_START,
            _ => LOROM_SRAM_START,
        };
        let sram = MemoryRegion::new(vec![0u8; header.sram_size_kb as usize * 1024], sram_base);
        Cartridge { header, sram, data }
    }

    pub fn read(&self, address: u32) -> u8 {
        match self.header.mapping_mode {
            MappingMode::LoRom => self.lorom_read(address),
            MappingMode::HiRom => self.hirom_read(address),
            MappingMode::ExHiRom => 0,
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match self.header.mapping_mode {
            MappingMode::LoRom => self.lorom_write(address, value),
            MappingMode::HiRom => self.hirom_write(address, value),
            MappingMode::ExHiRom => {}
        }
    }

    fn lorom_read(&self, address: u32) -> u8 {
        let (bank, offset) = Self::bank_and_offset(address);

        if let Some(sram_addr) = self.lorom_sram_addr(bank, offset) {
            return self.sram.read(&sram_addr);
        }

        if let Some(file_offset) = Self::lorom_file_offset(bank, offset)
            && file_offset < self.data.len()
        {
            return self.data[file_offset];
        }

        0
    }

    fn lorom_write(&mut self, address: u32, value: u8) {
        let (bank, offset) = Self::bank_and_offset(address);

        if let Some(sram_addr) = self.lorom_sram_addr(bank, offset) {
            self.sram.write(&sram_addr, value);
        }
    }

    fn hirom_read(&self, address: u32) -> u8 {
        let (bank, offset) = Self::bank_and_offset(address);

        if let Some(sram_addr) = self.hirom_sram_addr(bank, offset) {
            return self.sram.read(&sram_addr);
        }

        if let Some(file_offset) = Self::hirom_file_offset(bank, offset)
            && file_offset < self.data.len()
        {
            return self.data[file_offset];
        }

        0
    }

    fn hirom_write(&mut self, address: u32, value: u8) {
        let (bank, offset) = Self::bank_and_offset(address);

        if let Some(sram_addr) = self.hirom_sram_addr(bank, offset) {
            self.sram.write(&sram_addr, value);
        }
    }

    fn bank_and_offset(address: u32) -> (u8, u16) {
        ((address >> 16) as u8, (address & 0xFFFF) as u16)
    }

    fn lorom_sram_addr(&self, bank: u8, offset: u16) -> Option<u32> {
        if (0x70..=0x7D).contains(&bank) && offset < 0x8000 && self.header.sram_size_kb > 0 {
            let sram_size = self.header.sram_size_kb as usize * 1024;
            let raw_index = (bank - 0x70) as usize * 0x8000 + offset as usize;
            Some(LOROM_SRAM_START + (raw_index % sram_size) as u32)
        } else {
            None
        }
    }

    fn lorom_file_offset(bank: u8, offset: u16) -> Option<usize> {
        if offset >= 0x8000 {
            Some((bank & 0x7F) as usize * 0x8000 + (offset as usize - 0x8000))
        } else {
            None
        }
    }

    fn hirom_sram_addr(&self, bank: u8, offset: u16) -> Option<u32> {
        if (0x20..=0x3F).contains(&bank)
            && (0x6000..=0x7FFF).contains(&offset)
            && self.header.sram_size_kb > 0
        {
            let sram_size = self.header.sram_size_kb as usize * 1024;
            let raw_index = (bank - 0x20) as usize * 0x2000 + (offset - 0x6000) as usize;
            Some(HIROM_SRAM_START + (raw_index % sram_size) as u32)
        } else {
            None
        }
    }

    fn hirom_file_offset(bank: u8, offset: u16) -> Option<usize> {
        if bank >= 0xC0 {
            Some((bank - 0xC0) as usize * 0x10000 + offset as usize)
        } else if (0x40..=0x7F).contains(&bank) {
            Some((bank - 0x40) as usize * 0x10000 + offset as usize)
        } else if (0x80..=0xBF).contains(&bank) {
            Some((bank - 0x80) as usize * 0x10000 + offset as usize)
        } else if offset >= 0x8000 {
            Some(bank as usize * 0x10000 + offset as usize)
        } else {
            None
        }
    }

    fn strip_smc_header(data: Vec<u8>) -> Vec<u8> {
        if data.len() % 1024 == 512 {
            data[512..].to_vec()
        } else {
            data
        }
    }
}
