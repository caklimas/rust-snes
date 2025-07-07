use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode, read_byte, read_word, write_byte, write_word,
        },
    },
    memory::bus::Bus,
};

pub fn sta_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
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
    let cycles;
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

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sta_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let target_address = ((address_high as u16) << 8 | (address_low as u16)).into();
    let cycles;

    if is_8bit_mode(cpu) {
        write_byte(bus, target_address, (cpu.registers.a as u8) & 0xFF);
        cycles = 4;
    } else {
        write_word(bus, target_address, cpu.registers.a);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sta_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let base_address = ((address_high as u16) << 8 | (address_low as u16)) as u16;
    let target_address = (base_address + cpu.registers.x) as u32;
    let cycles;

    if is_8bit_mode(cpu) {
        write_byte(bus, target_address, (cpu.registers.a as u8) & 0xFF);
        cycles = 5;
    } else {
        write_word(bus, target_address, cpu.registers.a);
        cycles = 6;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sta_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let base_address = ((address_high as u16) << 8 | (address_low as u16)) as u16;
    let target_address = (base_address + cpu.registers.y) as u32;
    let cycles;

    if is_8bit_mode(cpu) {
        write_byte(bus, target_address, (cpu.registers.a as u8) & 0xFF);
        cycles = 5;
    } else {
        write_word(bus, target_address, cpu.registers.a);
        cycles = 6;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sta_indirect(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_byte(bus, (cpu.registers.pc + 1).into());
    let pointer_address = cpu.registers.d + (offset as u16);
    let target_address = read_word(bus, pointer_address.into()) as u32;
    let cycles;

    if is_8bit_mode(cpu) {
        write_byte(bus, target_address, (cpu.registers.a as u8) & 0xFF);
        cycles = 5;
    } else {
        write_word(bus, target_address, cpu.registers.a);
        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    cycles
}
