use std::{cell::RefCell, fmt, rc::Rc};

use crate::apu::{
    Apu,
    addresses::{CPU_IO_RANGE, CPU_IO_START, IO_PORTS_RANGE, IPL_BOOT_RANGE, IPL_BOOT_START},
    constants::IPL_ROM,
    io_ports::IoPorts,
    opcodes::execute_opcode,
    registers::Registers,
};

pub struct Spc700 {
    pub registers: Registers,
    apu: Rc<RefCell<Apu>>,
    io_ports: IoPorts,
    ipl_rom: [u8; 64],
    ram: [u8; 65536],
}

impl Spc700 {
    pub fn new(apu: Rc<RefCell<Apu>>) -> Self {
        Self {
            apu,
            io_ports: Default::default(),
            ipl_rom: IPL_ROM,
            ram: [0; 65536],
            registers: Default::default(),
        }
    }

    pub fn step(&mut self) {
        let opcode = self.read_byte();

        execute_opcode(self, opcode);
    }

    pub fn read_word_direct(&mut self, address: u32) -> u16 {
        let lo = self.read(address);
        let hi = self.read((address.wrapping_add(1)) & 0x00FF_FFFF);
        u16::from_le_bytes([lo, hi])
    }

    pub fn read_word(&mut self) -> u16 {
        let lo = self.read_byte();
        let hi = self.read_byte();
        u16::from_le_bytes([lo, hi])
    }

    pub fn read_byte(&mut self) -> u8 {
        let value = self.read(self.registers.pc as u32);
        self.registers.pc = self.registers.pc.wrapping_add(1);

        value
    }

    pub fn read(&mut self, address: u32) -> u8 {
        match (address, self.io_ports.control.ipl_rom_overlay()) {
            (addr, _) if CPU_IO_RANGE.contains(&addr) => {
                self.apu.borrow().cpu_to_spc[(addr - CPU_IO_START) as usize]
            }
            (addr, _) if IO_PORTS_RANGE.contains(&addr) => self.io_ports.read(address),
            (addr, true) if IPL_BOOT_RANGE.contains(&addr) => {
                self.ipl_rom[(addr - IPL_BOOT_START) as usize]
            }
            _ => self.ram[address as usize],
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            addr if CPU_IO_RANGE.contains(&addr) => {
                self.apu.borrow_mut().spc_to_cpu[(addr - CPU_IO_START) as usize] = value
            }
            addr if IO_PORTS_RANGE.contains(&addr) => self.io_ports.write(address, value),
            _ => self.ram[address as usize] = value,
        }
    }

    pub fn get_direct_page_address(&self, offset: u32) -> u32 {
        ((self.registers.psw.direct_page() as u32) * 0x100) | offset
    }

    pub fn set_z(&mut self, value: u8) {
        self.registers.psw.set_zero(value == 0);
    }

    pub fn set_n(&mut self, value: u8) {
        self.registers.psw.set_negative(value & 0x80 != 0);
    }

    pub fn set_c(&mut self, left: u8, right: u8) {
        self.registers.psw.set_carry(left >= right);
    }
}

impl fmt::Debug for Spc700 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Spc700")
            .field("registers", &self.registers)
            .finish()
    }
}
