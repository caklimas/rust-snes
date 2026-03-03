pub struct Oam {
    high_table: [u8; 32],
    low_table: [u8; 512],
    oamadd: u16,
    read_address: u16,
    write_latch: bool,
}

impl Oam {
    pub fn set_oamadd(&mut self, value: u8, is_low_bits: bool) {
        if is_low_bits {
            self.oamadd = (self.oamadd & 0xFF00) | (value as u16);
        } else {
            self.oamadd = (self.oamadd & 0xFF) | (((value & 0x1) as u16) << 8);
        }

        self.read_address = self.oamadd * 2;
        self.write_latch = false;
    }

    pub fn write_oamdata(&mut self, value: u8) {
        if self.oamadd <= 255 {
            let index = self.get_index(true);
            self.low_table[index] = value;
            if self.write_latch {
                self.oamadd += 1;
            }
        } else {
            let index = self.get_index(false);
            self.high_table[index] = value;

            if self.write_latch {
                self.oamadd = (self.oamadd + 1) % 272;
            }
        }

        self.write_latch = !self.write_latch;
    }

    pub fn read_oamdata(&mut self) -> u8 {
        let data = if self.read_address < 512 {
            self.low_table[(self.read_address) as usize]
        } else {
            self.high_table[(self.read_address - 512) as usize]
        };

        self.read_address = (self.read_address + 1) % 544;

        data
    }

    fn get_index(&self, is_low_table: bool) -> usize {
        let oamadd = if is_low_table {
            self.oamadd
        } else {
            self.oamadd - 256
        };

        if !self.write_latch {
            (oamadd * 2) as usize
        } else {
            ((oamadd * 2) + 1) as usize
        }
    }
}

impl Default for Oam {
    fn default() -> Self {
        Self {
            high_table: [0; 32],
            low_table: [0; 512],
            oamadd: 0,
            read_address: 0,
            write_latch: false,
        }
    }
}
