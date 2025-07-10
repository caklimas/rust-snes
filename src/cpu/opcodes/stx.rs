use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_x, read_byte, read_offset_byte, write_byte,
            write_word,
        },
    },
    memory::bus::Bus,
};

pub fn stx_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let target_address = (cpu.registers.d + offset) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        write_byte(bus, target_address, (cpu.registers.x as u8) & 0xFF);
        cycles = 3;
    } else {
        write_word(bus, target_address, cpu.registers.x);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn stx_direct_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let target_address = (cpu.registers.d + offset + cpu.registers.y) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        write_byte(bus, target_address, (cpu.registers.x as u8) & 0xFF);
        cycles = 4;
    } else {
        write_word(bus, target_address, cpu.registers.x);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn stx_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let target_address = ((address_high as u16) << 8 | (address_low as u16)).into();
    let cycles;

    if is_8bit_mode_x(cpu) {
        write_byte(bus, target_address, (cpu.registers.x as u8) & 0xFF);
        cycles = 4;
    } else {
        write_word(bus, target_address, cpu.registers.x);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}
