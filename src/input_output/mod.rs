use crate::{
    controller::Controller,
    memory::addresses::{JOY1H, JOY1L},
};

#[derive(Default)]
pub struct InputOutput {
    pub controller_1: Controller,
}

impl InputOutput {
    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            JOY1L => self.controller_1.low_byte() as u8,
            JOY1H => self.controller_1.high_byte() as u8,
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            _ => (),
        }
    }
}
