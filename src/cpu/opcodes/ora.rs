use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, calculate_direct_page_address,
            calculate_direct_page_x_address, calculate_indirect_page_address,
            calculate_indirect_page_x_address, calculate_indirect_page_y_address,
            calculate_stack_relative_address, calculate_stack_relative_indirect_y_address,
            direct_page_low_is_zero, get_address_absolute_x, get_address_absolute_x_data_physical,
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, page_crossed, read_byte,
            read_data_byte, read_data_byte_indirect_y, read_data_byte_stack_relative_indirect_y,
            read_data_word, read_data_word_indirect_y, read_data_word_stack_relative_indirect_y,
            read_long_pointer_direct_page, read_long_pointer_direct_page_wrapped, read_offset_byte,
            read_offset_word, read_phys_byte, read_phys_word, read_word, read_word_direct_page,
            set_nz_flags_u8, set_nz_flags_u16, stack_relative_indirect_y_dummy_read,
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
        perform_ora_u8(cpu, value);
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
    let source_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(source_address as u32);
        perform_ora_u8(cpu, value);
        3
    } else {
        let value = read_word_direct_page(bus, source_address);
        perform_ora_u16(cpu, value);
        4
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ora_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_absolute_long_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address);
        perform_ora_u8(cpu, value);
        5
    } else {
        let lo = bus.read(address);
        let hi = bus.read(address.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        perform_ora_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

pub fn ora_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_ora_u16(cpu, value);
        5
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // base = 16-bit operand, eff16 = low-16 effective, eff_phys = 24-bit effective (DBR:base + X)
    let (base, eff16, eff_phys) = get_address_absolute_x_data_physical(cpu, bus);

    // Perform the read + ORA using the *physical* effective address
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, eff_phys);
        perform_ora_u8(cpu, value);
        4
    } else {
        let value = read_phys_word(bus, eff_phys);
        perform_ora_u16(cpu, value);
        5
    };

    // - If index is 16-bit (X=0 in native): +1 always (do NOT also add page-cross)
    // - Else (index is 8-bit): +1 only on page-cross
    if !is_8bit_mode_x(cpu) {
        cycles += 1;
    } else if page_crossed(base, eff16) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ora_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base16 = read_offset_word(cpu, bus);

    // Y width for address math (emu => 8-bit)
    let y16: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let addr16 = base16.wrapping_add(y16);

    // Physical address per your harness: (DBR:base16) + Y in 24-bit space
    let base_phys = ((cpu.registers.db as u32) << 16) | (base16 as u32);
    let phys = (base_phys.wrapping_add(y16 as u32)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys);
        perform_ora_u8(cpu, value);
        4
    } else {
        let lo = bus.read(phys);
        let hi = bus.read((phys.wrapping_add(1)) & 0x00FF_FFFF);
        perform_ora_u16(cpu, u16::from_le_bytes([lo, hi]));
        5
    };

    // Only add +1 on 16-bit page cross
    if !is_8bit_mode_x(cpu) || page_crossed(base16, addr16) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ora_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address) = calculate_indirect_page_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address16) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let (value, extra) = read_data_byte_indirect_y(cpu, bus, base_address, address16);
        perform_ora_u8(cpu, value);
        5 + (extra as u8)
    } else {
        let (value, extra) = read_data_word_indirect_y(cpu, bus, base_address, address16);
        perform_ora_u16(cpu, value);
        6 + (extra as u8)
    };

    // Direct page low-byte penalty
    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_ora_u8(cpu, value);
        5
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_ora_u16(cpu, value);
        6
    };
    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);
    let effective = read_long_pointer_direct_page(bus, address);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(effective);
        perform_ora_u8(cpu, value);
        6
    } else {
        let lo = bus.read(effective);
        let hi = bus.read(effective.wrapping_add(1));
        let value = u16::from_le_bytes([lo, hi]);
        perform_ora_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_stack_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_stack_relative_address(cpu, bus);

    if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let a = (cpu.registers.a & 0x00FF) as u8;
        let result = a | value;

        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);

        increment_program_counter(cpu, 2);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        let result = cpu.registers.a | value;

        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);

        increment_program_counter(cpu, 2);
        5 // typical +1 when M=0
    }
}

pub fn ora_stack_relative_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pointer_address, base_address, _effective16) =
        calculate_stack_relative_indirect_y_address(cpu, bus);

    stack_relative_indirect_y_dummy_read(cpu, bus, pointer_address);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte_stack_relative_indirect_y(cpu, bus, base_address);
        perform_ora_u8(cpu, value);
        7
    } else {
        let value = read_data_word_stack_relative_indirect_y(cpu, bus, base_address);
        perform_ora_u16(cpu, value);
        8
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ora_indirect_long_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
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
        perform_ora_u8(cpu, value);
        6
    } else {
        let lo = bus.read(phys);
        let hi = bus.read((phys.wrapping_add(1)) & 0x00FF_FFFF);
        perform_ora_u16(cpu, u16::from_le_bytes([lo, hi]));
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

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
