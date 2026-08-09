#[derive(Default)]
pub struct Register16 {
    pub lo: u8,
    pub hi: u8,
}

impl Register16 {
    pub fn value(&self) -> u16 {
        u16::from_le_bytes([self.lo, self.hi])
    }

    pub fn set(&mut self, val: u16) {
        self.lo = val as u8;
        self.hi = (val >> 8) as u8;
    }
}
