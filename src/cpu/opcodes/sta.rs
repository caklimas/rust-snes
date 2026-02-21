use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            calculate_indirect_page_address, calculate_indirect_page_x_address,
            calculate_indirect_page_y_address, get_x_register_value, increment_program_counter,
            is_8bit_mode_m, read_program_byte, write_data_byte, write_data_word,
        },
    },
    memory::MemoryBus,
};

// STA - Store Accumulator
// Stores the accumulator value to memory. Does not affect any processor flags.

pub fn sta_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        3
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_base_address, target_address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        4
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
    let target_address = (address_high as u16) << 8 | (address_low as u16);

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        4
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sta_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
    let base_address = (address_high as u16) << 8 | (address_low as u16);
    let target_address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        5
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sta_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
    let base_address = (address_high as u16) << 8 | (address_low as u16);
    let target_address = base_address + cpu.registers.y;

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        5
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sta_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_indirect_page_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        5
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, pointer_address) = calculate_indirect_page_x_address(cpu, bus);
    let target_address_low = read_program_byte(cpu, bus, pointer_address) as u16;
    let target_address_high = read_program_byte(cpu, bus, pointer_address + 1) as u16;
    let target_address = (target_address_high << 8) | target_address_low;

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        6
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, target_address) = calculate_indirect_page_y_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        6
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}
