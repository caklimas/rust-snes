pub struct Cgram {
    cgadd: u16,
    data: [u8; 512],
    read_latch: bool,
    write_latch: bool,
}

impl Cgram {
    pub fn read_color(&self, index: u8) -> u16 {
        let index = (index * 2) as usize;
        let lo = self.data[index];
        let hi = self.data[index + 1];

        u16::from_le_bytes([lo, hi])
    }

    pub fn write_cgadd(&mut self, value: u8) {
        self.cgadd = (value as u16) * 2;
        self.write_latch = false;
        self.read_latch = false;
    }

    pub fn write_cgdata(&mut self, value: u8) {
        let index = self.get_index(self.write_latch);
        if !self.write_latch {
            self.data[index] = value;
        } else {
            self.data[index] = value & 0x7F;

            self.increment_cgadd();
        }

        self.write_latch = !self.write_latch;
    }

    pub fn read_cgdata(&mut self) -> u8 {
        let index = self.get_index(self.read_latch);
        if self.read_latch {
            self.increment_cgadd();
        }

        self.read_latch = !self.read_latch;
        self.data[index]
    }

    fn get_index(&self, latch: bool) -> usize {
        (if latch { self.cgadd + 1 } else { self.cgadd }) as usize
    }

    fn increment_cgadd(&mut self) {
        self.cgadd = (self.cgadd + 2) % 512;
    }
}

impl Default for Cgram {
    fn default() -> Self {
        Self {
            cgadd: 0,
            data: [0; 512],
            read_latch: false,
            write_latch: false,
        }
    }
}
