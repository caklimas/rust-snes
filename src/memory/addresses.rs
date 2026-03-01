use std::ops::RangeInclusive;

pub const NMI_STATUS_REGISTER: u32 = 0x004210;

// Direct Page (Zero Page) - controlled by D register, default 0x000000
pub const DIRECT_PAGE_START: u32 = 0x000000;
pub const DIRECT_PAGE_END: u32 = 0x0000FF;
pub const DIRECT_PAGE_RANGE: RangeInclusive<u32> = DIRECT_PAGE_START..=DIRECT_PAGE_END;

// Stack area (Bank 0 only)
pub const STACK_START: u32 = 0x000100;
pub const STACK_END: u32 = 0x001FFF; // 6502 emulation mode (page 1 only)
pub const STACK_RANGE: RangeInclusive<u32> = STACK_START..=STACK_END;

// WRAM ranges
pub const WRAM_START: u32 = 0x7E0000;
pub const WRAM_END: u32 = 0x7FFFFF;
pub const WRAM_RANGE: RangeInclusive<u32> = WRAM_START..=WRAM_END;

// WRAM mirror: first 8KB mirrored at $00:0000–$1FFF in banks $00–$3F and $80–$BF
pub const WRAM_MIRROR_OFFSET_START: u16 = 0x0000;
pub const WRAM_MIRROR_OFFSET_END: u16 = 0x1FFF;

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

// Reset vectors
pub const RESET_VECTOR_LO: u32 = 0x00FFFC;
pub const RESET_VECTOR_HI: u32 = 0x00FFFD;

// Cartridge SRAM - LoROM: $70:0000–$77:FFFF
pub const LOROM_SRAM_START: u32 = 0x700000;
pub const LOROM_SRAM_END: u32 = 0x77FFFF;
pub const LOROM_SRAM_RANGE: RangeInclusive<u32> = LOROM_SRAM_START..=LOROM_SRAM_END;

// Cartridge SRAM - HiROM: $20:6000–$3F:7FFF
pub const HIROM_SRAM_START: u32 = 0x206000;
pub const HIROM_SRAM_END: u32 = 0x3F7FFF;
pub const HIROM_SRAM_RANGE: RangeInclusive<u32> = HIROM_SRAM_START..=HIROM_SRAM_END;
