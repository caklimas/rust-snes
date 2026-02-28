use crate::{
    cpu::Cpu,
    memory::{
        addresses::{RESET_VECTOR_HI, RESET_VECTOR_LO},
        bus::Bus,
    },
};

pub struct SuperNintendo {
    bus: Bus,
    cpu: Cpu,
}

impl SuperNintendo {
    pub fn new(data: Vec<u8>) -> Self {
        let mut bus = Bus::new(data);
        let mut cpu = Cpu::default();

        let lo = bus.read(RESET_VECTOR_LO);
        let hi = bus.read(RESET_VECTOR_HI);
        cpu.registers.pc = u16::from_le_bytes([lo, hi]);

        Self { bus, cpu }
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step(&mut self.bus)
    }
}
