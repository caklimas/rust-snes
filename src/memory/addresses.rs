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

// Unused B-Bus gap
pub const UNUSED_IO_GAP_START: u32 = 0x002000;
pub const UNUSED_IO_GAP_END: u32 = 0x0020FF;
pub const UNUSED_IO_GAP_RANGE: RangeInclusive<u32> = UNUSED_IO_GAP_START..=UNUSED_IO_GAP_END;

// PPU register ranges
pub const PPU_REGISTERS_START: u32 = 0x002100;
pub const PPU_REGISTERS_END: u32 = 0x00213F;
pub const PPU_REGISTERS_RANGE: RangeInclusive<u32> = PPU_REGISTERS_START..=PPU_REGISTERS_END;

pub const INIDISP: u32 = 0x002100;
pub const BGMODE: u32 = 0x002105;

pub const BG1SC: u32 = 0x002107;
pub const BG2SC: u32 = 0x002108;
pub const BG3SC: u32 = 0x002109;
pub const BG4SC: u32 = 0x00210A;

pub const BG12NBA: u32 = 0x00210B;
pub const BG34NBA: u32 = 0x00210C;

pub const BG1HOFS: u32 = 0x00210D;
pub const BG1VOFS: u32 = 0x00210E;
pub const BG2HOFS: u32 = 0x00210F;
pub const BG2VOFS: u32 = 0x002110;
pub const BG3HOFS: u32 = 0x002111;
pub const BG3VOFS: u32 = 0x002112;
pub const BG4HOFS: u32 = 0x002113;
pub const BG4VOFS: u32 = 0x002114;

pub const TM: u32 = 0x00212C;
pub const TS: u32 = 0x00212D;

pub const SETINI: u32 = 0x002133;

// OAM
pub const OAMADD_LO: u32 = 0x002102;
pub const OAMADD_HI: u32 = 0x002103;
pub const OAMDATA: u32 = 0x002104;
pub const OAMDATAREAD: u32 = 0x002138;

// VRAM
pub const VMAIN: u32 = 0x002115;
pub const VMADDL: u32 = 0x002116;
pub const VMADDH: u32 = 0x002117;
pub const VMDATAL: u32 = 0x002118;
pub const VMDATAH: u32 = 0x002119;

// CGRAM
pub const CGADD: u32 = 0x002121;
pub const CGDATA: u32 = 0x002122;
pub const CGDATAREAD: u32 = 0x00213B;

// APU register ranges
pub const APU_REGISTERS_START: u32 = 0x002140;
pub const APU_REGISTERS_END: u32 = 0x00217F;
pub const APU_REGISTERS_RANGE: RangeInclusive<u32> = APU_REGISTERS_START..=APU_REGISTERS_END;

// WRAM access ports
pub const WMDATA: u32 = 0x002180;
pub const WMADDL: u32 = 0x002181;
pub const WMADDM: u32 = 0x002182;
pub const WMADDH: u32 = 0x002183;

// Unused upper gap
pub const UNUSED_UPPER_GAP_START: u32 = 0x002184;
pub const UNUSED_UPPER_GAP_END: u32 = 0x003FFF;
pub const UNUSED_UPPER_GAP_RANGE: RangeInclusive<u32> =
    UNUSED_UPPER_GAP_START..=UNUSED_UPPER_GAP_END;

// Joypad IO
pub const JOYPAD_IO_START: u32 = 0x004000;
pub const JOYPAD_IO_END: u32 = 0x0041FF;
pub const JOYPAD_IO_RANGE: RangeInclusive<u32> = JOYPAD_IO_START..=JOYPAD_IO_END;

// CPU IO
pub const CPU_IO_START: u32 = 0x004200;
pub const CPU_IO_END: u32 = 0x005FFF;
pub const CPU_IO_RANGE: RangeInclusive<u32> = CPU_IO_START..=CPU_IO_END;

pub const NMITIMEN: u32 = 0x004200;

// DMA
pub const MDMAEN: u32 = 0x00420B;
pub const DMA_REGISTERS_START: u32 = 0x004300;
pub const DMA_REGISTERS_END: u32 = 0x00437F;
pub const DMA_REGISTERS_RANGE: RangeInclusive<u32> = DMA_REGISTERS_START..=DMA_REGISTERS_END;

// Cartridge ROM (LoROM example)
pub const CARTRIDGE_ROM_START: u32 = 0x008000;
pub const CARTRIDGE_ROM_END: u32 = 0x00FFFF;
pub const CARTRIDGE_ROM_RANGE: RangeInclusive<u32> = CARTRIDGE_ROM_START..=CARTRIDGE_ROM_END;

pub const NMI_VECTOR_NATIVE_LO: u32 = 0x00FFEA;
pub const NMI_VECTOR_NATIVE_HI: u32 = 0x00FFEB;

pub const NMI_VECTOR_EMULATOR_LO: u32 = 0x00FFFA;
pub const NMI_VECTOR_EMULATOR_HI: u32 = 0x00FFFB;

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
