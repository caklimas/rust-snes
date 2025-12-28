use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, calculate_direct_page_address,
            calculate_direct_page_x_address, calculate_indirect_page_address,
            calculate_indirect_page_x_address, calculate_indirect_page_y_address,
            calculate_stack_relative_indirect_y_address, direct_page_low_is_zero,
            get_address_absolute_x, get_address_absolute_x_data_physical,
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, page_crossed, read_byte,
            read_data_byte, read_data_byte_indirect_y, read_data_byte_stack_relative_indirect_y,
            read_data_word, read_data_word_indirect_y, read_data_word_stack_relative_indirect_y,
            read_long_pointer_direct_page, read_offset_byte, read_offset_word, read_phys_byte,
            read_phys_word, read_word, read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
            stack_relative_indirect_y_dummy_read,
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
        perform_and_u8(cpu, value);
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

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(source_address as u32);
        perform_and_u8(cpu, value);
        3
    } else {
        let value = read_word_direct_page(bus, source_address);
        perform_and_u16(cpu, value);
        4
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn and_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_and_u16(cpu, value);
        5
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base, eff16, phys) = get_address_absolute_x_data_physical(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, phys);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_phys_word(bus, phys);
        perform_and_u16(cpu, value);
        5
    };

    if !is_8bit_mode_x(cpu) {
        cycles += 1;
    } else if page_crossed(base, eff16) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn and_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base16 = read_offset_word(cpu, bus);

    let y16: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let addr16 = base16.wrapping_add(y16);

    let base_phys = ((cpu.registers.db as u32) << 16) | (base16 as u32);
    let phys = (base_phys.wrapping_add(y16 as u32)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, phys);
        perform_and_u8(cpu, value);
        4
    } else {
        let value = read_phys_word(bus, phys);
        perform_and_u16(cpu, value);
        5
    };

    if !cpu.emulation_mode && !is_8bit_mode_x(cpu) {
        cycles += 1;
    } else if page_crossed(base16, addr16) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn and_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address) = calculate_indirect_page_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address16) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let (value, extra) = read_data_byte_indirect_y(cpu, bus, base_address, address16);
        perform_and_u8(cpu, value);
        5 + if extra { 1 } else { 0 }
    } else {
        let (value, extra) = read_data_word_indirect_y(cpu, bus, base_address, address16);
        perform_and_u16(cpu, value);
        6 + if extra { 1 } else { 0 }
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_and_u8(cpu, value);
        5
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_and_u16(cpu, value);
        6
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_stack_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // 8-bit offset from instruction stream
    let offset = read_offset_byte(cpu, bus);

    // In emulation mode, stack addressing uses $01xx
    let s_for_addressing: u16 = if cpu.emulation_mode {
        0x0100 | (cpu.registers.s & 0x00FF)
    } else {
        cpu.registers.s
    };

    // Effective address is in bank 0
    let addr = s_for_addressing.wrapping_add(offset as u16);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(addr as u32); // bank 0
        perform_and_u8(cpu, value);
        4
    } else {
        let lo = bus.read(addr as u32);
        let hi = bus.read(addr.wrapping_add(1) as u32);
        perform_and_u16(cpu, u16::from_le_bytes([lo, hi]));
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_direct_page_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // dp address = D + immediate offset (bank 0)
    let dp_addr = calculate_direct_page_address(cpu, bus);

    // For [dp], your harness expects NO special DP wrapping for the 3-byte pointer fetch.
    // Read 24-bit pointer from bank 0 at dp_addr..dp_addr+2.
    let phys = read_long_pointer_direct_page(bus, dp_addr) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, phys);
        perform_and_u8(cpu, value);
        6
    } else {
        let value = read_phys_word(bus, phys);
        perform_and_u16(cpu, value);
        7
    };

    // +1 cycle if D.l != 0 for direct page addressing
    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let phys = calculate_absolute_long_address(cpu, bus) & 0x00FF_FFFF;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, phys);
        perform_and_u8(cpu, value);
        5
    } else {
        let value = read_phys_word(bus, phys);
        perform_and_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

pub fn and_stack_relative_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pointer_address, base_address, _effective16) =
        calculate_stack_relative_indirect_y_address(cpu, bus);

    stack_relative_indirect_y_dummy_read(cpu, bus, pointer_address);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte_stack_relative_indirect_y(cpu, bus, base_address);
        perform_and_u8(cpu, value);
        7
    } else {
        let value = read_data_word_stack_relative_indirect_y(cpu, bus, base_address);
        perform_and_u16(cpu, value);
        8
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn and_direct_page_indirect_long_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_addr = calculate_direct_page_address(cpu, bus);

    let base_phys = read_long_pointer_direct_page(bus, dp_addr) & 0x00FF_FFFF;

    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y & 0x00FF) as u32
    } else {
        cpu.registers.y as u32
    };

    let eff_phys = (base_phys.wrapping_add(y)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, eff_phys);
        perform_and_u8(cpu, value);
        6
    } else {
        let value = read_phys_word(bus, eff_phys);
        perform_and_u16(cpu, value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

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
