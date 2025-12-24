use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, increment_program_counter, is_8bit_mode_x, read_byte,
            read_offset_byte, read_offset_word, read_word, set_nz_flags_u8, set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// CPY - Compare Y Register with Memory
// Compares the Y register with a value from memory by performing Y - M. Sets N, Z, and C flags but does not modify the Y register.
// The carry flag is set if Y >= M (unsigned comparison). Commonly used before conditional branches in loops.

pub fn cpy_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_x(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_compare_u8(cpu, value as u16);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_compare_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn cpy_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);
    let cycles = if is_8bit_mode_x(cpu) {
        let value = bus.read(address as u32);
        perform_compare_u8(cpu, value as u16);
        3
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_u16(cpu, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cpy_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_x(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_compare_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_compare_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

fn perform_compare_u8(cpu: &mut Cpu, value: u16) {
    let y_value = cpu.registers.y & 0xFF;
    let result = y_value.wrapping_sub(value);

    set_nz_flags_u8(cpu, (result & 0xFF) as u8);
    set_c_flag_u8(cpu, y_value, value);
}

fn set_c_flag_u8(cpu: &mut Cpu, y_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, y_value >= value);
}

fn perform_compare_u16(cpu: &mut Cpu, value: u16) {
    let result = cpu.registers.y.wrapping_sub(value);

    set_nz_flags_u16(cpu, result);
    set_c_flag_u16(cpu, cpu.registers.y, value);
}

fn set_c_flag_u16(cpu: &mut Cpu, y_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, y_value >= value);
}
