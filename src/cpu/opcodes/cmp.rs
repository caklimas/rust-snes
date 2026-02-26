use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, calculate_absolute_long_x_address,
            calculate_absolute_x_address, calculate_stack_relative_address,
            calculate_absolute_y_address, calculate_direct_page_address,
            calculate_direct_page_x_address, calculate_indirect_page_address,
            calculate_indirect_page_x_address, calculate_indirect_page_y_address,
            calculate_stack_relative_indirect_y_address, direct_page_low_is_zero,
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, page_crossed,
            read_data_byte, read_data_byte_indirect_y,
            read_data_byte_stack_relative_indirect_y, read_data_word,
            read_data_word_indirect_y, read_data_word_stack_relative_indirect_y,
            read_long_pointer_direct_page, read_long_pointer_direct_page_wrapped,
            read_offset_byte, read_offset_word, read_word_direct_page,
            set_nz_flags_u8, set_nz_flags_u16, stack_relative_indirect_y_dummy_read,
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

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        perform_compare_with_carry_u8(cpu, value as u16);
        3
    } else {
        let value = read_word_direct_page(bus, address);
        perform_compare_with_carry_u16(cpu, value);
        4
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn cmp_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = calculate_absolute_x_address(cpu, bus);
    let index = address.wrapping_sub(base_address);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = bus.read_word( phys);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn cmp_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = calculate_absolute_y_address(cpu, bus);
    let index = address.wrapping_sub(base_address);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys) as u16;
        perform_compare_with_carry_u8(cpu, value);
        4
    } else {
        let value = bus.read_word( phys);
        perform_compare_with_carry_u16(cpu, value);
        5
    };

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn cmp_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address) = calculate_indirect_page_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address16) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let (value, extra) = read_data_byte_indirect_y(cpu, bus, base_address, address16);
        perform_compare_with_carry_u8(cpu, value as u16);
        5 + (extra as u8)
    } else {
        let (value, extra) = read_data_word_indirect_y(cpu, bus, base_address, address16);
        perform_compare_with_carry_u16(cpu, value);
        6 + (extra as u8)
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn cmp_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address) as u16;
        perform_compare_with_carry_u8(cpu, value);
        5
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_compare_with_carry_u16(cpu, value);
        6
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0xD7 - CMP Direct Page Indirect Long Indexed Y: [dp],Y
pub fn cmp_direct_page_indirect_long_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_base = calculate_direct_page_address(cpu, bus);
    let base_phys = read_long_pointer_direct_page_wrapped(cpu, bus, dp_base);

    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y & 0x00FF) as u32
    } else {
        cpu.registers.y as u32
    };

    let phys = base_phys.wrapping_add(y) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys);
        perform_compare_with_carry_u8(cpu, value as u16);
        6
    } else {
        let lo = bus.read(phys);
        let hi = bus.read(phys.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        perform_compare_with_carry_u16(cpu, value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0xD3 - CMP Stack Relative Indirect Indexed Y: (sr,S),Y
pub fn cmp_stack_relative_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pointer_address, base_address, _) =
        calculate_stack_relative_indirect_y_address(cpu, bus);

    stack_relative_indirect_y_dummy_read(cpu, bus, pointer_address);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte_stack_relative_indirect_y(cpu, bus, base_address);
        perform_compare_with_carry_u8(cpu, value as u16);
        7
    } else {
        let value = read_data_word_stack_relative_indirect_y(cpu, bus, base_address);
        perform_compare_with_carry_u16(cpu, value);
        8
    };

    increment_program_counter(cpu, 2);
    cycles
}

// 0xCF - CMP Absolute Long: addr_long
pub fn cmp_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_absolute_long_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address);
        perform_compare_with_carry_u8(cpu, value as u16);
        5
    } else {
        let lo = bus.read(address);
        let hi = bus.read(address.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        perform_compare_with_carry_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

// 0xC3 - CMP Stack Relative: sr,S
pub fn cmp_stack_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_stack_relative_address(cpu, bus);

    if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32) as u16;
        perform_compare_with_carry_u8(cpu, value);
        increment_program_counter(cpu, 2);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_compare_with_carry_u16(cpu, value);
        increment_program_counter(cpu, 2);
        5
    }
}

// 0xDF - CMP Absolute Long Indexed X: addr_long,X
pub fn cmp_absolute_long_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, effective_phys) = calculate_absolute_long_x_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(effective_phys);
        perform_compare_with_carry_u8(cpu, value as u16);
        5
    } else {
        let lo = bus.read(effective_phys);
        let hi = bus.read(effective_phys.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        perform_compare_with_carry_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

// 0xC7 - CMP Direct Page Indirect Long: [dp]
pub fn cmp_direct_page_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_addr = calculate_direct_page_address(cpu, bus);
    let effective = read_long_pointer_direct_page(bus, dp_addr);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(effective);
        perform_compare_with_carry_u8(cpu, value as u16);
        6
    } else {
        let lo = bus.read(effective);
        let hi = bus.read(effective.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        perform_compare_with_carry_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

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
