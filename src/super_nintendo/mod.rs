use crate::{cpu::Cpu, memory::bus::Bus};

pub struct SuperNintendo {
    bus: Bus,
    cpu: Cpu,
}

impl SuperNintendo {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            bus: Bus::new(data),
            cpu: Cpu::default(),
        }
    }

    pub fn step(&mut self) -> u8 {
        self.cpu.step(&mut self.bus)
    }
}
