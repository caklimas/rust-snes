pub struct MemoryRegion {
    data: Vec<u8>,
    base_address: u32,
}

impl MemoryRegion {
    pub fn new(data: Vec<u8>, base_address: u32) -> Self {
        Self { data, base_address }
    }

    pub fn read(&self, address: &u32) -> u8 {
        self.data[self.get_index(address)]
    }

    pub fn write(&mut self, address: &u32, data: u8) {
        let index = self.get_index(address);
        self.data[index] = data;
    }

    fn get_index(&self, address: &u32) -> usize {
        (address - self.base_address) as usize
    }
}
