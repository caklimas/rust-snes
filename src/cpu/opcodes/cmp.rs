use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_address_absolute_x, get_address_absolute_y, get_address_indirect_x,
            increment_program_counter, is_8bit_mode_m, page_crossed, read_byte, read_offset_byte,
            read_offset_word, read_word, set_nz_flags_u8, set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

pub fn cmp_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let program_increment;

    if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_compare_with_carry_u8(cpu, value);

        program_increment = 2;
        cycles = 2;
    } else {
        let value = read_offset_word(cpu, bus);
        perform_compare_with_carry_u16(cpu, value);

        program_increment = 3;
        cycles = 3;
    }

    increment_program_counter(cpu, program_increment);

    cycles
}

pub fn cmp_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_compare_with_carry_u8(cpu, value as u16);

        cycles = 3;
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);

        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn cmp_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let address = read_offset_word(cpu, bus);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn cmp_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + cpu.registers.x;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn cmp_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (base_address, address) = get_address_absolute_x(cpu, bus);
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    cycles
}

pub fn cmp_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (base_address, address) = get_address_absolute_y(cpu, bus);
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    cycles
}

pub fn cmp_indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let address = get_address_indirect_x(cpu, bus);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);

        cycles = 6;
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);

        cycles = 7;
    }

    increment_program_counter(cpu, 2);

    cycles
}

fn perform_compare_with_carry_u8(cpu: &mut Cpu, value: u16) {
    let accumulator_value = cpu.registers.a & 0xFF;
    let result = accumulator_value - value;

    set_nz_flags_u8(cpu, (result & 0xFF) as u8);
    set_c_flag_u8(cpu, accumulator_value, value);
}

fn set_c_flag_u8(cpu: &mut Cpu, accumulator_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, accumulator_value >= value);
}

fn perform_compare_with_carry_u16(cpu: &mut Cpu, value: u16) {
    let result = value - cpu.registers.a;

    set_nz_flags_u16(cpu, result);
    set_c_flag_u16(cpu, cpu.registers.a, value);
}

fn set_c_flag_u16(cpu: &mut Cpu, accumulator_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, accumulator_value >= value);
}
