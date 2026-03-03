use crate::ppu::vmain::Vmain;

pub struct Vram {
    pub vmain: Vmain,
    data: [u8; 65536],
    address_register: u16,
}

impl Vram {
    pub fn set_address_lo(&mut self, value: u8) {
        self.address_register = (self.address_register & 0xFF00) | (value as u16);
    }

    pub fn set_address_hi(&mut self, value: u8) {
        self.address_register = (self.address_register & 0x00FF) | (((value & 0x7F) as u16) << 8);
    }

    pub fn write_data_lo(&mut self, value: u8) {
        let address = self.address_register * 2;
        self.data[address as usize] = value;

        self.increment_address_register(!self.vmain.increment_timing());
    }

    pub fn write_data_hi(&mut self, value: u8) {
        let address = (self.address_register * 2) + 1;
        self.data[address as usize] = value;

        self.increment_address_register(self.vmain.increment_timing());
    }

    fn increment_address_register(&mut self, increment: bool) {
        if increment {
            self.address_register = self
                .address_register
                .wrapping_add(self.get_increment_amount());
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
            vmain: Default::default(),
        }
    }
}
