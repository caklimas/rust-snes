use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            calculate_indirect_page_address, calculate_indirect_page_x_address,
            calculate_indirect_page_y_address, get_address_absolute_x, get_address_absolute_y,
            increment_program_counter, is_8bit_mode_m, page_crossed, read_byte, read_offset_byte,
            read_offset_word, read_word, read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// CMP - Compare Accumulator with Memory
// Compares the accumulator with a value from memory by performing A - M. Sets N, Z, and C flags but does not modify the accumulator.
// The carry flag is set if A >= M (unsigned comparison). Commonly used before conditional branches.

pub fn cmp_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (program_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_compare_with_carry_u8(cpu, value as u16);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_compare_with_carry_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, program_increment);
    cycles
}

pub fn cmp_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        perform_compare_with_carry_u8(cpu, value as u16);
        3
    } else {
        let value = read_word_direct_page(bus, address);
        perform_compare_with_carry_u16(cpu, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn cmp_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = get_address_absolute_x(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn cmp_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = get_address_absolute_y(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn cmp_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address) = calculate_indirect_page_x_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        6
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        6
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

fn perform_compare_with_carry_u8(cpu: &mut Cpu, value: u16) {
    let accumulator_value = cpu.registers.a & 0x00FF;
    let v = value & 0x00FF;

    let result = accumulator_value.wrapping_sub(v);

    set_nz_flags_u8(cpu, (result & 0x00FF) as u8);
    set_c_flag_u8(cpu, accumulator_value, v);
}

fn set_c_flag_u8(cpu: &mut Cpu, accumulator_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, accumulator_value >= value);
}

fn perform_compare_with_carry_u16(cpu: &mut Cpu, value: u16) {
    let accumulator_value = cpu.registers.a;
    let result = accumulator_value.wrapping_sub(value);

    set_nz_flags_u16(cpu, result);
    set_c_flag_u16(cpu, accumulator_value, value);
}

fn set_c_flag_u16(cpu: &mut Cpu, accumulator_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, accumulator_value >= value);
}
