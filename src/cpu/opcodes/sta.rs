use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_x_register_value, increment_program_counter, is_8bit_mode_m, read_byte,
            read_offset_byte, read_word, write_byte, write_word,
        },
    },
    memory::bus::Bus,
};

// STA - Store Accumulator
// Stores the accumulator value to memory. Does not affect any processor flags.

pub fn sta_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let target_address = cpu.registers.d + offset;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 3;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sta_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let base_address = cpu.registers.d + offset;
    let target_address = base_address + get_x_register_value(cpu);

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 4;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sta_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(cpu, bus, (cpu.registers.pc + 1));
    let address_high = read_byte(cpu, bus, (cpu.registers.pc + 2));
    let target_address = (address_high as u16) << 8 | (address_low as u16);
    let cycles;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 4;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sta_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(cpu, bus, (cpu.registers.pc + 1));
    let address_high = read_byte(cpu, bus, (cpu.registers.pc + 2));
    let base_address = ((address_high as u16) << 8 | (address_low as u16)) as u16;
    let target_address = base_address + get_x_register_value(cpu);
    let cycles;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 5;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 6;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sta_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(cpu, bus, (cpu.registers.pc + 1));
    let address_high = read_byte(cpu, bus, (cpu.registers.pc + 2));
    let base_address = ((address_high as u16) << 8 | (address_low as u16)) as u16;
    let target_address = base_address + cpu.registers.y;
    let cycles;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 5;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 6;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sta_indirect(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    let target_address = read_word(cpu, bus, pointer_address);
    let cycles;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 5;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sta_indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let base_pointer_address = cpu.registers.d + offset;
    let pointer_address = base_pointer_address + get_x_register_value(cpu);

    let target_address_low = read_byte(cpu, bus, pointer_address) as u16;
    let target_address_high = read_byte(cpu, bus, pointer_address + 1) as u16;
    let target_address = (target_address_high << 8) | target_address_low;
    let cycles;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 6;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 7;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sta_indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    let base_address = read_word(cpu, bus, pointer_address);
    let target_address = base_address + cpu.registers.y;
    let cycles;

    if is_8bit_mode_m(cpu) {
        write_byte(cpu, bus, target_address, ((cpu.registers.a as u8)));
        cycles = 6;
    } else {
        write_word(cpu, bus, target_address, cpu.registers.a);
        cycles = 7;
    }

    increment_program_counter(cpu, 2);

    cycles
}
