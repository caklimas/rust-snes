use std::ops::RangeInclusive;

pub const IO_PORTS_START: u32 = 0x00F0;
pub const IO_PORTS_END: u32 = 0x00FF;
pub const IO_PORTS_RANGE: RangeInclusive<u32> = IO_PORTS_START..=IO_PORTS_END;

pub const IO_PORT_TEST: u32 = 0x00F0;
pub const IO_PORT_CONTROL: u32 = 0x00F1;
pub const DSPADDR: u32 = 0x00F2;
pub const DSPDATA: u32 = 0x00F3;

pub const AUX_1: u32 = 0x00F8;
pub const AUX_2: u32 = 0x00F9;

pub const TIMER_DIVIDER_1: u32 = 0x00FA;
pub const TIMER_DIVIDER_3: u32 = 0x00FC;

pub const TIMER_COUNTER_1: u32 = 0x00FD;
pub const TIMER_COUNTER_3: u32 = 0x00FF;

pub const IPL_BOOT_START: u32 = 0xFFC0;
pub const IPL_BOOT_END: u32 = 0xFFFF;
pub const IPL_BOOT_RANGE: RangeInclusive<u32> = IPL_BOOT_START..=IPL_BOOT_END;

pub const CPU_IO_START: u32 = 0x00F4;
pub const CPU_IO_END: u32 = 0x00F7;
pub const CPU_IO_RANGE: RangeInclusive<u32> = CPU_IO_START..=CPU_IO_END;
