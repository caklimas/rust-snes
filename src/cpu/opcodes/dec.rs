use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_x_register_value, increment_program_counter, is_8bit_mode_m, is_8bit_mode_x,
            read_byte, read_offset_byte, read_offset_word, read_word, set_nz_flags_u8,
            set_nz_flags_u16, write_byte, write_word,
        },
    },
    memory::bus::Bus,
};

// DEC - Decrement Memory
// Subtracts 1 from the value at a memory location. Sets N and Z flags based on the result.
// Commonly used for decrementing counters, loop indices, and other memory-based values.

pub fn dec_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn dec_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn dec_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn dec_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let result = value.wrapping_sub(1);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// DEA - Decrement Accumulator
// Subtracts 1 from the accumulator. Sets N and Z flags based on the result.
// Commonly used for simple accumulator-based counting.
pub fn dea(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let result = value.wrapping_sub(1);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let result = cpu.registers.a.wrapping_sub(1);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

// DEX - Decrement X Register
// Subtracts 1 from the X register. Sets N and Z flags based on the result.
// Commonly used in loops and array indexing.
pub fn dex(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.x & 0xFF) as u8;
        let result = value.wrapping_sub(1);
        cpu.registers.x = (cpu.registers.x & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let result = cpu.registers.x.wrapping_sub(1);
        cpu.registers.x = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

// DEY - Decrement Y Register
// Subtracts 1 from the Y register. Sets N and Z flags based on the result.
// Commonly used in loops and array indexing.
pub fn dey(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.y & 0xFF) as u8;
        let result = value.wrapping_sub(1);
        cpu.registers.y = (cpu.registers.y & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let result = cpu.registers.y.wrapping_sub(1);
        cpu.registers.y = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}
