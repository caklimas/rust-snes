use crate::memory::dma_parameter::DmaParameter;

#[derive(Clone, Copy, Default)]
pub struct DmaChannel {
    pub dmap: DmaParameter,
    pub bbad: u8,
    pub a1t: u16,
    pub a1b: u8,
    pub das: u16,
    pub hdma_table_ptr: u16,
    pub hdma_line_counter: u8,
    pub hdma_do_transfer: bool,
}

impl DmaChannel {
    pub fn set_register(&mut self, lower_nibble: u8, value: u8) {
        match lower_nibble {
            0 => self.dmap.0 = value,
            1 => self.bbad = value,
            2 => self.a1t = (self.a1t & 0xFF00) | (value as u16),
            3 => self.a1t = (self.a1t & 0x00FF) | ((value as u16) << 8),
            4 => self.a1b = value,
            5 => self.das = (self.das & 0xFF00) | (value as u16),
            6 => self.das = (self.das & 0x00FF) | ((value as u16) << 8),
            _ => {}
        }
    }
}
