use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            increment_program_counter, is_8bit_mode_m, read_offset_word, write_data_byte,
            write_data_word,
        },
    },
    memory::MemoryBus,
};

// STZ - Store Zero to Memory
// Stores a zero value to memory. More efficient than loading zero into A and then storing.
// Does not affect any flags.
pub fn stz_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, address, 0);
        3
    } else {
        write_data_word(cpu, bus, address, 0);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn stz_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, address, 0);
        4
    } else {
        write_data_word(cpu, bus, address, 0);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn stz_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, address, 0);
        4
    } else {
        write_data_word(cpu, bus, address, 0);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn stz_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.x;

    let cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, address, 0);
        5
    } else {
        write_data_word(cpu, bus, address, 0);
        6
    };

    increment_program_counter(cpu, 3);
    cycles
}
