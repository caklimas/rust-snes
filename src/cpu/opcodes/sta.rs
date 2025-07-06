use crate::{
    cpu::{
        Cpu,
        opcodes::{increment_program_counter, is_8bit_mode, read_byte, write_byte, write_word},
    },
    memory::bus::Bus,
};

pub fn sta_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut cycles = 0;
    let offset = read_byte(bus, (cpu.registers.pc + 1) as u32);
    let target_address = (cpu.registers.d + (offset as u16)) as u32;

    if is_8bit_mode(cpu) {
        write_byte(bus, target_address, (cpu.registers.a as u8) & 0xFF);
        cycles = 3;
    } else {
        write_word(bus, target_address, cpu.registers.a);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sta_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut cycles = 0;
    let offset = read_byte(bus, (cpu.registers.pc + 1) as u32);
    let base_address = (cpu.registers.d + (offset as u16)) as u32;
    let target_address = base_address + cpu.registers.x as u32;

    if is_8bit_mode(cpu) {
        write_byte(bus, target_address, (cpu.registers.a as u8) & 0xFF);
        cycles = 4;
    } else {
        write_word(bus, target_address, cpu.registers.a);
        cycles = 5;
    }
    cycles
}
