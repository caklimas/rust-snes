use std::fmt;

use bitfield::bitfield;

bitfield! {

    #[derive(Clone, Copy, Default)]
    pub struct ProcessorStatusWord(u8);

    pub carry, set_carry: 0;
    pub zero, set_zero: 1;
    pub interrupt_enable, set_interrupt_enable: 2;
    pub half_carry, set_half_carry: 3;
    pub brk, set_brk: 4;
    pub direct_page, set_direct_page: 5;
    pub overflow, set_overflow: 6;
    pub negative, set_negative: 7;
}

impl fmt::Debug for ProcessorStatusWord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{}{}{}{}{}{} (0x{:02X})",
            if self.negative() { 'N' } else { '-' },
            if self.overflow() { 'V' } else { '-' },
            if self.direct_page() { 'P' } else { '-' },
            if self.brk() { 'B' } else { '-' },
            if self.half_carry() { 'H' } else { '-' },
            if self.interrupt_enable() { 'I' } else { '-' },
            if self.zero() { 'Z' } else { '-' },
            if self.carry() { 'C' } else { '-' },
            self.0,
        )
    }
}
