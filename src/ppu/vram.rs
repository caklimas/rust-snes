use crate::ppu::vmain::Vmain;

pub struct Vram {
    pub vmain: Vmain,
    pub rendering_active: bool,
    data: [u8; 65536],
    address_register: u16,
    prefetch_buffer: u16,
}

impl Vram {
    pub fn address_register(&self) -> u16 {
        self.address_register
    }

    pub fn read(&self, address: usize) -> u8 {
        self.data[address]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let index = (address * 2) as usize;
        let lo = self.data[index];
        let hi = self.data[index + 1];

        u16::from_le_bytes([lo, hi])
    }

    pub fn set_address_lo(&mut self, value: u8) {
        self.address_register = (self.address_register & 0xFF00) | (value as u16);
    }

    pub fn set_address_hi(&mut self, value: u8) {
        self.address_register = (self.address_register & 0x00FF) | (((value & 0x7F) as u16) << 8);
        self.prefetch();
    }

    pub fn read_data_lo(&mut self) -> u8 {
        let value = self.prefetch_buffer as u8;
        self.increment_address_register(!self.vmain.increment_timing());
        value
    }

    pub fn read_data_hi(&mut self) -> u8 {
        let value = (self.prefetch_buffer >> 8) as u8;
        self.increment_address_register(self.vmain.increment_timing());
        value
    }

    pub fn write_data_lo(&mut self, value: u8, write_data: bool) {
        if write_data {
            let address = self.address_register * 2;
            self.data[address as usize] = value;
        }

        self.increment_address_register(!self.vmain.increment_timing());
    }

    pub fn write_data_hi(&mut self, value: u8, write_data: bool) {
        if write_data {
            let address = (self.address_register * 2) + 1;
            self.data[address as usize] = value;
        }

        self.increment_address_register(self.vmain.increment_timing());
    }

    fn prefetch(&mut self) {
        self.prefetch_buffer = self.read_word(self.address_register);
    }

    fn increment_address_register(&mut self, increment: bool) {
        if increment {
            self.address_register = self
                .address_register
                .wrapping_add(self.get_increment_amount());
            self.prefetch();
        }
    }

    fn get_increment_amount(&self) -> u16 {
        match self.vmain.increment_amount() {
            0 => 1,
            1 => 32,
            2 | 3 => 128,
            _ => unreachable!(),
        }
    }
}

impl Default for Vram {
    fn default() -> Self {
        Self {
            data: [0; 65536],
            address_register: Default::default(),
            prefetch_buffer: 0,
            vmain: Default::default(),
            rendering_active: true,
        }
    }
}
