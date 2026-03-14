use crate::memory::addresses::{APU_REGISTERS_RANGE, APU_REGISTERS_START};

pub struct Apu {
    cpu_to_spc: [u8; 4],
    spc_to_cpu: [u8; 4],
}

impl Apu {
    pub fn read(&self, address: u32) -> u8 {
        match address {
            addr if APU_REGISTERS_RANGE.contains(&addr) => {
                let index = self.get_index(address);
                self.spc_to_cpu[index]
            }
            _ => {
                eprintln!("Unhandled APU read: {:#06X}", address);
                0
            }
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            addr if APU_REGISTERS_RANGE.contains(&addr) => {
                let index = self.get_index(address);
                self.spc_to_cpu[index] = value;
                self.cpu_to_spc[index] = value;
            }
            _ => {
                eprintln!("Unhandled APU write: {:#06X}", address);
            }
        }
    }

    fn get_index(&self, address: u32) -> usize {
        ((address - APU_REGISTERS_START) % 4) as usize
    }
}

impl Default for Apu {
    fn default() -> Self {
        Self {
            cpu_to_spc: [0x00; 4],
            spc_to_cpu: [0xAA, 0xBB, 0x00, 0x00],
        }
    }
}
