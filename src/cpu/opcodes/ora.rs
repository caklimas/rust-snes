use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_address_absolute_x, get_address_indirect, get_address_indirect_x,
            get_address_indirect_y, get_x_register_value, increment_program_counter,
            is_8bit_mode_m, page_crossed, read_byte, read_offset_byte, read_offset_word, read_word,
            set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::MemoryBus,
};

// ORA - Logical OR with Accumulator
// Performs a bitwise OR between the accumulator and a value from memory, storing the result in the accumulator.
// Sets N and Z flags based on the result. Commonly used for setting specific bits or combining bit patterns.

pub fn ora_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_ora_u8(cpu, value.try_into().unwrap());
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_ora_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn ora_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let source_address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, source_address);
        perform_ora_u8(cpu, value);
        3
    } else {
        let value = read_word(cpu, bus, source_address);
        perform_ora_u16(cpu, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ora_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = get_address_absolute_x(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ora_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.y;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ora_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = get_address_indirect_x(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        6
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = get_address_indirect_y(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        6
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = get_address_indirect(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

fn perform_ora_u8(cpu: &mut Cpu, value: u8) {
    let result = (cpu.registers.a & 0xFF) | (value as u16);
    cpu.registers.a = (cpu.registers.a & 0xFF00) | result;
    set_nz_flags_u8(cpu, result as u8);
}

fn perform_ora_u16(cpu: &mut Cpu, value: u16) {
    let result = cpu.registers.a | value;
    cpu.registers.a = result;
    set_nz_flags_u16(cpu, result);
}
