const LOROM_HEADER_OFFSET: usize = 0x7FC0;
const HIROM_HEADER_OFFSET: usize = 0xFFC0;

const TITLE_LENGTH: usize = 21;

#[derive(Debug, Clone, PartialEq)]
pub enum MappingMode {
    LoRom,
    HiRom,
    ExHiRom,
}

#[derive(Debug)]
pub struct CartridgeHeader {
    pub title: String,
    pub mapping_mode: MappingMode,
    pub rom_size_kb: u32,
    pub sram_size_kb: u32,
}

impl CartridgeHeader {
    pub fn new(data: &[u8]) -> Self {
        let lorom_score = Self::score_header(data, LOROM_HEADER_OFFSET);
        let hirom_score = Self::score_header(data, HIROM_HEADER_OFFSET);

        let offset = if hirom_score > lorom_score {
            HIROM_HEADER_OFFSET
        } else {
            LOROM_HEADER_OFFSET
        };

        Self::parse(data, offset)
    }

    fn score_header(data: &[u8], offset: usize) -> u32 {
        if data.len() < offset + 0x20 {
            return 0;
        }

        let mut score = 0u32;

        let checksum_complement = u16::from_le_bytes([data[offset + 0x1C], data[offset + 0x1D]]);
        let checksum = u16::from_le_bytes([data[offset + 0x1E], data[offset + 0x1F]]);

        if checksum.wrapping_add(checksum_complement) == 0xFFFF {
            score += 2;
        }

        let mapping_byte = data[offset + 0x15];
        if matches!(mapping_byte, 0x20 | 0x21 | 0x25) {
            score += 1;
        }

        score
    }

    fn parse(data: &[u8], offset: usize) -> Self {
        let title_bytes = &data[offset..offset + TITLE_LENGTH];
        let title = String::from_utf8_lossy(title_bytes)
            .trim_end_matches('\0')
            .trim()
            .to_string();

        let mapping_mode = match data[offset + 0x15] {
            0x21 => MappingMode::HiRom,
            0x25 => MappingMode::ExHiRom,
            _ => MappingMode::LoRom,
        };

        let rom_size_byte = data[offset + 0x17];
        let rom_size_kb = if rom_size_byte > 0 {
            1u32 << rom_size_byte
        } else {
            0
        };

        let sram_size_byte = data[offset + 0x18];
        let sram_size_kb = if sram_size_byte > 0 {
            1u32 << sram_size_byte
        } else {
            0
        };

        let checksum_complement = u16::from_le_bytes([data[offset + 0x1C], data[offset + 0x1D]]);
        let checksum = u16::from_le_bytes([data[offset + 0x1E], data[offset + 0x1F]]);

        if checksum.wrapping_add(checksum_complement) != 0xFFFF {
            eprintln!(
                "Warning: ROM checksum invalid (checksum: {:#06X}, complement: {:#06X})",
                checksum, checksum_complement
            );
        }

        Self {
            title,
            mapping_mode,
            rom_size_kb,
            sram_size_kb,
        }
    }
}
