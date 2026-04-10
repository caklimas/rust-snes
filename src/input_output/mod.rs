use crate::{
    controller::Controller,
    input_output::register16::Register16,
    memory::addresses::{
        JOY1H, JOY1L, RDDIVH, RDDIVL, RDMPYH, RDMPYL, WRDIVB, WRDIVH, WRDIVL, WRMPYA, WRMPYB,
    },
};

pub mod register16;

#[derive(Default)]
pub struct InputOutput {
    pub controller_1: Controller,
    wrmpya: u8,
    dividend: Register16,
    quotient: Register16,
    math_result: Register16,
}

impl InputOutput {
    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            JOY1L => self.controller_1.low_byte() as u8,
            JOY1H => self.controller_1.high_byte() as u8,
            RDDIVL => self.quotient.lo,
            RDDIVH => self.quotient.hi,
            RDMPYL => self.math_result.lo,
            RDMPYH => self.math_result.hi,
            _ => 0,
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            WRMPYA => self.wrmpya = value,
            WRMPYB => {
                let product = self.wrmpya as u16 * value as u16;
                self.math_result.set(product);
            }
            WRDIVL => self.dividend.lo = value,
            WRDIVH => self.dividend.hi = value,
            WRDIVB => {
                if value == 0 {
                    self.quotient.set(0xFFFF);
                    self.math_result.set(self.dividend.value());
                } else {
                    let dividend = self.dividend.value();
                    self.quotient.set(dividend / value as u16);
                    self.math_result.set(dividend % value as u16);
                }
            }
            _ => {}
        }
    }
}
