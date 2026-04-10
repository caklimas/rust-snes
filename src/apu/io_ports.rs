use crate::apu::{
    addresses::{
        AUX_1, AUX_2, DSPADDR, DSPDATA, IO_PORT_CONTROL, IO_PORT_TEST, TIMER_COUNTER_1,
        TIMER_COUNTER_3, TIMER_DIVIDER_1, TIMER_DIVIDER_3,
    },
    control::Control,
    timer::Timer,
};

pub struct IoPorts {
    pub aux: [u8; 2],
    pub control: Control,
    pub dsp_address: u8,
    pub dsp_registers: [u8; 128],
    timers: [Timer; 3],
}

impl IoPorts {
    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            IO_PORT_TEST => 0,
            IO_PORT_CONTROL => self.control.0,
            DSPADDR => self.dsp_address,
            DSPDATA => self.dsp_registers[self.get_dsp_address()],
            AUX_1 | AUX_2 => self.aux[self.get_aux_address(address)],
            TIMER_DIVIDER_1..=TIMER_DIVIDER_3 => 0,
            TIMER_COUNTER_1..=TIMER_COUNTER_3 => {
                let value = self.timers[self.get_timer_counter_address(address)].counter;
                self.timers[self.get_timer_counter_address(address)].counter = 0;

                value & 0x0F
            }
            _ => unimplemented!(),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            IO_PORT_TEST => {}
            IO_PORT_CONTROL => self.control.0 = value,
            DSPADDR => self.dsp_address = value,
            DSPDATA => self.dsp_registers[self.get_dsp_address()] = value,
            AUX_1..=AUX_2 => self.aux[self.get_aux_address(address)] = value,
            TIMER_DIVIDER_1..=TIMER_DIVIDER_3 => {
                self.timers[self.get_timer_divider_address(address)].divider = value
            }
            TIMER_COUNTER_1..=TIMER_COUNTER_3 => (),
            _ => unimplemented!(),
        }
    }

    fn get_dsp_address(&self) -> usize {
        (self.dsp_address & 0x7F) as usize
    }

    fn get_aux_address(&self, address: u32) -> usize {
        (address - AUX_1) as usize
    }

    fn get_timer_divider_address(&self, address: u32) -> usize {
        (address - TIMER_DIVIDER_1) as usize
    }

    fn get_timer_counter_address(&self, address: u32) -> usize {
        (address - TIMER_COUNTER_1) as usize
    }
}

impl Default for IoPorts {
    fn default() -> Self {
        Self {
            aux: [0; 2],
            control: Default::default(),
            dsp_address: Default::default(),
            dsp_registers: [0; 128],
            timers: Default::default(),
        }
    }
}
