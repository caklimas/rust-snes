use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_addresses,
            get_address_absolute_x, get_address_indirect_y, get_x_register_value,
            increment_program_counter, is_8bit_mode_m, page_crossed, read_byte, read_offset_byte,
            read_offset_word, read_word, read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::MemoryBus,
};

// LDA - Load Accumulator
// Loads a value from memory into the accumulator register. Sets N and Z flags based on the loaded value.
// Supports 8-bit and 16-bit modes depending on the M flag.

// LDA (0xA5) - Direct Page
// Loads from memory at (Direct Page + offset).
pub fn lda_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(target_address as u32);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        3
    } else {
        let value = read_word_direct_page(bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

// LDA (0xB5) - Direct Page Indexed by X
// Loads from memory at (Direct Page + offset + X).
pub fn lda_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, target_address) = calculate_direct_page_x_addresses(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(target_address as u32);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 2);

    if page_crossed(target_address, base_address) {
        cycles += 1;
    }

    cycles
}

// LDA (0xA9) - Immediate
// Loads an immediate value directly from the instruction stream into the accumulator.
pub fn lda_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = cpu.registers.pc + 1;

    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        (2, 2)
    } else {
        let value = read_word(cpu, bus, address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

// LDA (0xAD) - Absolute
// Loads from a 16-bit absolute memory address.
pub fn lda_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

// LDA (0xBD) - Absolute Indexed by X
// Loads from memory at (absolute address + X).
pub fn lda_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, target_address) = get_address_absolute_x(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);

    if page_crossed(base_address, target_address) {
        cycles += 1;
    }

    cycles
}

// LDA (0xB9) - Absolute Indexed by Y
// Loads from memory at (absolute address + Y).
pub fn lda_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offet = read_offset_word(cpu, bus);
    let target_address = offet + cpu.registers.y;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);

    if page_crossed(offet, target_address) {
        cycles += 1;
    }

    cycles
}

// LDA (0xB2) - Direct Page Indirect
// Loads from the address stored at (Direct Page + offset).
pub fn lda_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    let target_address = read_word(cpu, bus, pointer_address);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

// LDA (0xA1) - Direct Page Indexed Indirect
// Loads from the address stored at (Direct Page + offset + X).
pub fn lda_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let base_pointer_address = cpu.registers.d + offset;
    let pointer_address = base_pointer_address + get_x_register_value(cpu);
    let target_address = read_word(cpu, bus, pointer_address);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        6
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        7
    };

    increment_program_counter(cpu, 2);

    if page_crossed(base_pointer_address, pointer_address) {
        cycles += 1;
    }

    cycles
}

// LDA (0xB1) - Direct Page Indirect Indexed by Y
// Loads from the address (stored at Direct Page + offset) + Y.
pub fn lda_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, target_address) = get_address_indirect_y(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        5
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 2);

    if page_crossed(base_address, target_address) {
        cycles += 1;
    }

    cycles
}

fn set_accumulator_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
}

fn set_accumulator_u16(cpu: &mut Cpu, value: u16) {
    cpu.registers.a = value;
}
