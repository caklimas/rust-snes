use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, calculate_absolute_long_x_address,
            calculate_absolute_x_address, calculate_direct_page_address,
            calculate_direct_page_x_address, calculate_indirect_page_address,
            calculate_indirect_page_x_address, calculate_indirect_page_y_address,
            calculate_stack_relative_address, calculate_stack_relative_indirect_y_address,
            direct_page_low_is_zero, increment_program_counter, is_8bit_mode_m, is_8bit_mode_x,
            page_crossed, read_data_byte, read_data_byte_indirect_y,
            read_data_byte_stack_relative_indirect_y, read_data_word, read_data_word_indirect_y,
            read_data_word_stack_relative_indirect_y, read_long_pointer_direct_page,
            read_long_pointer_direct_page_wrapped, read_offset_word, read_program_byte,
            read_program_word, read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
            stack_relative_indirect_y_dummy_read,
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

    let mut cycles = if is_8bit_mode_m(cpu) {
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

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// LDA (0xB5) - Direct Page Indexed by X
// Loads from memory at (Direct Page + offset + X).
pub fn lda_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, target_address) = calculate_direct_page_x_address(cpu, bus);
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

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// LDA (0xA9) - Immediate
// Loads an immediate value directly from the instruction stream into the accumulator.
pub fn lda_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = cpu.registers.pc + 1;

    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_program_byte(cpu, bus, address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        (2, 2)
    } else {
        let value = read_program_word(cpu, bus, address);
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
        let value = read_data_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = read_data_word(cpu, bus, target_address);
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
    let (base_address, target_address) = calculate_absolute_x_address(cpu, bus);
    let index = target_address.wrapping_sub(base_address);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = bus.read_word(phys);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, target_address) {
        cycles += 1;
    }

    cycles
}

// LDA (0xB9) - Absolute Indexed by Y
// Loads from memory at (absolute address + Y).
pub fn lda_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offet = read_offset_word(cpu, bus);
    let target_address = offet.wrapping_add(cpu.registers.y);
    let phys = (((cpu.registers.db as u32) << 16) + (offet as u32) + (cpu.registers.y as u32))
        & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = bus.read_word(phys);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);

    if !is_8bit_mode_x(cpu) || page_crossed(offet, target_address) {
        cycles += 1;
    }

    cycles
}

// LDA (0xB2) - Direct Page Indirect
// Loads from the address stored at (Direct Page + offset).
pub fn lda_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_indirect_page_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        5
    } else {
        let value = read_data_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        6
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// LDA (0xA1) - Direct Page Indexed Indirect
// Loads from the address stored at (Direct Page + offset + X).
pub fn lda_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, target_address) = calculate_indirect_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        6
    } else {
        let value = read_data_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// LDA (0xB1) - Direct Page Indirect Indexed by Y
// Loads from the address (stored at Direct Page + offset) + Y.
pub fn lda_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address16) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let (value, extra) = read_data_byte_indirect_y(cpu, bus, base_address, address16);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        5 + (extra as u8)
    } else {
        let (value, extra) = read_data_word_indirect_y(cpu, bus, base_address, address16);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        6 + (extra as u8)
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0xA3 - LDA Stack Relative
pub fn lda_stack_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_stack_relative_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

// 0xA7 - LDA Direct Page Indirect Long: [dp]
pub fn lda_direct_page_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_addr = calculate_direct_page_address(cpu, bus);
    let effective = read_long_pointer_direct_page(bus, dp_addr);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(effective);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        6
    } else {
        let lo = bus.read(effective);
        let hi = bus.read(effective.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0xAF - LDA Absolute Long
pub fn lda_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_absolute_long_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        5
    } else {
        let lo = bus.read(address);
        let hi = bus.read(address.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

// 0xB3 - LDA Stack Relative Indirect Indexed Y: (sr,S),Y
pub fn lda_stack_relative_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pointer_address, base_address, _) = calculate_stack_relative_indirect_y_address(cpu, bus);

    stack_relative_indirect_y_dummy_read(cpu, bus, pointer_address);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte_stack_relative_indirect_y(cpu, bus, base_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        7
    } else {
        let value = read_data_word_stack_relative_indirect_y(cpu, bus, base_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        8
    };

    increment_program_counter(cpu, 2);
    cycles
}

// 0xB7 - LDA Direct Page Indirect Long Indexed Y: [dp],Y
pub fn lda_direct_page_indirect_long_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_base = calculate_direct_page_address(cpu, bus);
    let base_phys = read_long_pointer_direct_page_wrapped(cpu, bus, dp_base);

    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y & 0x00FF) as u32
    } else {
        cpu.registers.y as u32
    };

    let phys = (base_phys.wrapping_add(y)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        6
    } else {
        let value = bus.read_word(phys);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0xBF - LDA Absolute Long Indexed X: addr_long,X
pub fn lda_absolute_long_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, eff_phys) = calculate_absolute_long_x_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(eff_phys);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        5
    } else {
        let value = bus.read_word(eff_phys);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

fn set_accumulator_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
}

fn set_accumulator_u16(cpu: &mut Cpu, value: u16) {
    cpu.registers.a = value;
}
