use crate::apu::{
    addresses::{CPU_IO_RANGE, IO_PORTS_RANGE, IPL_BOOT_RANGE, IPL_BOOT_START},
    io_ports::IoPorts,
    registers::Registers,
};

pub struct Spc700 {
    pub io_ports: IoPorts,
    pub ipl_rom: [u8; 64],
    pub ram: [u8; 65536],
    pub registers: Registers,
}

impl Spc700 {
    pub fn read(&mut self, address: u32) -> u8 {
        match (address, self.io_ports.control.ipl_rom_overlay()) {
            (addr, _) if CPU_IO_RANGE.contains(&addr) => 0,
            (addr, _) if IO_PORTS_RANGE.contains(&addr) => self.io_ports.read(address),
            (addr, true) if IPL_BOOT_RANGE.contains(&addr) => {
                self.ipl_rom[(addr - IPL_BOOT_START) as usize]
            }
            _ => self.ram[address as usize],
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            addr if CPU_IO_RANGE.contains(&addr) => (),
            addr if IO_PORTS_RANGE.contains(&addr) => self.io_ports.write(address, value),
            _ => self.ram[address as usize] = value,
        }
    }
}

impl Default for Spc700 {
    fn default() -> Self {
        Self {
            io_ports: Default::default(),
            ipl_rom: [0; 64],
            ram: [0; 65536],
            registers: Default::default(),
        }
    }
}
