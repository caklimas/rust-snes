use crate::apu::{addresses::IO_PORT_CONTROL, control::Control};

#[derive(Default)]
pub struct IoPorts {
    pub control: Control,
}

impl IoPorts {
    pub fn read(&self, address: u32) -> u8 {
        match address {
            IO_PORT_CONTROL => self.control.0,
            _ => unimplemented!(),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            IO_PORT_CONTROL => self.control.0 = value,
            _ => unimplemented!(),
        }
    }
}
