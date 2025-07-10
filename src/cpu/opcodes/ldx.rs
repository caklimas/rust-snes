use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_x, page_crossed, read_byte, read_offset_byte,
            read_offset_word, read_word, set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::bus::Bus,
};

pub fn ldx_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let mut pc_increment = 2;
    if is_8bit_mode_x(cpu) {
        let value = read_offset_byte(cpu, bus);
        cpu.registers.x = value;
        set_nz_flags_u8(cpu, value as u8);
        cycles = 2;
    } else {
        let value = read_offset_word(cpu, bus);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        pc_increment += 1;
        cycles = 3;
    }

    increment_program_counter(cpu, pc_increment);

    cycles
}

pub fn ldx_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let source_address = (cpu.registers.d + offset) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(bus, source_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word(bus, source_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn ldx_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let absolute_address = read_offset_word(cpu, bus) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(bus, absolute_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word(bus, absolute_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn ldx_direct_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let source_address = (cpu.registers.d + offset + cpu.registers.y) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(bus, source_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(bus, source_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn ldx_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let target_address = (base_address + cpu.registers.y) as u32;
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(bus, target_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(bus, target_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if page_crossed(base_address, target_address as u16) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}
