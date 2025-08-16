use crate::{
    cpu::{
        Cpu,
        opcodes::{get_address_absolute_long, push_byte, read_offset_word},
    },
    memory::bus::Bus,
};

pub fn jsr_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);
    let return_address = cpu.registers.pc + 2;

    push_word(cpu, bus, return_address);

    cpu.registers.pc = address;

    6
}

pub fn jsr_absolute_long(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let target_address = get_address_absolute_long(cpu, bus);
    let return_address = cpu.registers.pc + 3;

    push_byte(cpu, bus, cpu.registers.pb);
    push_word(cpu, bus, return_address);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    8
}

fn push_word(cpu: &mut Cpu, bus: &mut Bus, address: u16) {
    push_byte(cpu, bus, ((address >> 8) & 0xFF) as u8);
    push_byte(cpu, bus, (address & 0xFF) as u8);
}
