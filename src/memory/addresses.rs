use std::ops::RangeInclusive;

pub const NMI_STATUS_REGISTER: u32 = 0x004210;

// WRAM ranges
pub const WRAM_START: u32 = 0x7E0000;
pub const WRAM_END: u32 = 0x7FFFFF;
pub const WRAM_RANGE: RangeInclusive<u32> = WRAM_START..=WRAM_END;

// PPU register ranges
pub const PPU_REGISTERS_START: u32 = 0x002100;
pub const PPU_REGISTERS_END: u32 = 0x00213F;
pub const PPU_REGISTERS_RANGE: RangeInclusive<u32> = PPU_REGISTERS_START..=PPU_REGISTERS_END;

// APU register ranges
pub const APU_REGISTERS_START: u32 = 0x002140;
pub const APU_REGISTERS_END: u32 = 0x00217F;
pub const APU_REGISTERS_RANGE: RangeInclusive<u32> = APU_REGISTERS_START..=APU_REGISTERS_END;

// Cartridge ROM (LoROM example)
pub const CARTRIDGE_ROM_START: u32 = 0x008000;
pub const CARTRIDGE_ROM_END: u32 = 0x00FFFF;
pub const CARTRIDGE_ROM_RANGE: RangeInclusive<u32> = CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END;
