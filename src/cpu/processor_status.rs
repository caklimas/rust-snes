use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ProcessorStatus: u8 {
        const CARRY        = 0b00000001; // C
        const ZERO         = 0b00000010; // Z
        const IRQ_DISABLE  = 0b00000100; // I
        const DECIMAL      = 0b00001000; // D
        const INDEX_WIDTH  = 0b00010000; // X (0=16-bit, 1=8-bit)
        const MEMORY_WIDTH = 0b00100000; // M (0=16-bit, 1=8-bit)
        const OVERFLOW     = 0b01000000; // V
        const NEGATIVE     = 0b10000000; // N
    }
}
