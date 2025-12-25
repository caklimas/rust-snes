use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            calculate_indirect_page_address, calculate_indirect_page_x_address,
            calculate_indirect_page_y_address, get_address_absolute_x, get_x_register_value,
            increment_program_counter, is_8bit_mode_m, page_crossed, read_byte, read_offset_byte,
            read_offset_word, read_word, read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::MemoryBus,
};

// AND - Logical AND with Accumulator
// Performs a bitwise AND between the accumulator and a value from memory, storing the result in the accumulator.
// Sets N and Z flags based on the result. Commonly used for bit masking and testing specific bits.

pub fn and_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_and_u8(cpu, value.try_into().unwrap());
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_and_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn and_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(source_address as u32);
        perform_and_u8(cpu, value);
        3
    } else {
        let value = read_word_direct_page(bus, source_address);
        perform_and_u16(cpu, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn and_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_and_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = get_address_absolute_x(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn and_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.y;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn and_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_x_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        6
    } else {
        let value = read_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        6
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

fn perform_and_u8(cpu: &mut Cpu, value: u8) {
    let result = (cpu.registers.a & 0xFF) & (value as u16);
    cpu.registers.a = (cpu.registers.a & 0xFF00) | result;
    set_nz_flags_u8(cpu, result as u8);
}

fn perform_and_u16(cpu: &mut Cpu, value: u16) {
    let result = cpu.registers.a & value;
    cpu.registers.a = result;
    set_nz_flags_u16(cpu, result);
}
