use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, calculate_absolute_long_x_address,
            calculate_direct_page_address, calculate_direct_page_x_address,
            calculate_indirect_page_address, calculate_indirect_page_x_address,
            calculate_indirect_page_y_address, calculate_stack_relative_address,
            calculate_stack_relative_indirect_y_address, direct_page_low_is_zero,
            effective_phys_indirect_y, effective_phys_stack_relative_indirect_y,
            get_x_register_value, increment_program_counter, is_8bit_mode_m, is_8bit_mode_x,
            read_long_pointer_direct_page, read_long_pointer_direct_page_wrapped, read_program_byte,
            stack_relative_indirect_y_dummy_read, write_byte_direct_page, write_data_byte,
            write_data_word, write_word_direct_page,
        },
    },
    memory::MemoryBus,
};

// STA - Store Accumulator
// Stores the accumulator value to memory. Does not affect any processor flags.

pub fn sta_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        write_byte_direct_page(bus, target_address, cpu.registers.a as u8);
        3
    } else {
        write_word_direct_page(bus, target_address, cpu.registers.a);
        4
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_base_address, target_address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        write_byte_direct_page(bus, target_address, cpu.registers.a as u8);
        4
    } else {
        write_word_direct_page(bus, target_address, cpu.registers.a);
        5
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(1));
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));
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
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(1));
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));
    let base_address = (address_high as u16) << 8 | (address_low as u16);
    let index = get_x_register_value(cpu);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;

    let cycles = if is_8bit_mode_m(cpu) {
        bus.write(phys, cpu.registers.a as u8);
        5
    } else {
        bus.write_word( phys, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sta_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(1));
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));
    let base_address = (address_high as u16) << 8 | (address_low as u16);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (cpu.registers.y as u32))
            & 0x00FF_FFFF;

    let cycles = if is_8bit_mode_m(cpu) {
        bus.write(phys, cpu.registers.a as u8);
        5
    } else {
        bus.write_word( phys, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sta_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_indirect_page_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        5
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        6
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, target_address) = calculate_indirect_page_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        6
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, _) = calculate_indirect_page_y_address(cpu, bus);
    let phys = effective_phys_indirect_y(cpu, base_address);
    let mut cycles = if is_8bit_mode_m(cpu) {
        bus.write(phys, cpu.registers.a as u8);
        6
    } else {
        bus.write_word( phys, cpu.registers.a);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0x83 - STA Stack Relative: S+offset, bank 0
pub fn sta_stack_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_stack_relative_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        bus.write(address as u32, cpu.registers.a as u8);
        4
    } else {
        bus.write_word(address as u32, cpu.registers.a);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

// 0x87 - STA Direct Page Indirect Long: [dp] (24-bit pointer from bank 0)
pub fn sta_direct_page_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_addr = calculate_direct_page_address(cpu, bus);
    let effective = read_long_pointer_direct_page(bus, dp_addr);

    let mut cycles = if is_8bit_mode_m(cpu) {
        bus.write(effective, cpu.registers.a as u8);
        6
    } else {
        bus.write_word(effective, cpu.registers.a);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0x8F - STA Absolute Long: 24-bit address from operand
pub fn sta_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_absolute_long_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        bus.write(address, cpu.registers.a as u8);
        5
    } else {
        bus.write_word(address, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}

// 0x93 - STA Stack Relative Indirect Indexed Y: (sr,S),Y
pub fn sta_stack_relative_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pointer_address, base_address, _) =
        calculate_stack_relative_indirect_y_address(cpu, bus);

    stack_relative_indirect_y_dummy_read(cpu, bus, pointer_address);

    let phys = effective_phys_stack_relative_indirect_y(cpu, base_address);

    let cycles = if is_8bit_mode_m(cpu) {
        bus.write(phys, cpu.registers.a as u8);
        7
    } else {
        bus.write_word(phys, cpu.registers.a);
        8
    };

    increment_program_counter(cpu, 2);
    cycles
}

// 0x97 - STA Direct Page Indirect Long Indexed Y: [dp],Y
pub fn sta_direct_page_indirect_long_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_base = calculate_direct_page_address(cpu, bus);
    let base_phys = read_long_pointer_direct_page_wrapped(cpu, bus, dp_base);

    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y & 0x00FF) as u32
    } else {
        cpu.registers.y as u32
    };

    let phys = (base_phys.wrapping_add(y)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        bus.write(phys, cpu.registers.a as u8);
        6
    } else {
        bus.write_word(phys, cpu.registers.a);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// 0x9F - STA Absolute Long Indexed X: addr_long,X
pub fn sta_absolute_long_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, eff_phys) = calculate_absolute_long_x_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        bus.write(eff_phys, cpu.registers.a as u8);
        5
    } else {
        bus.write_word(eff_phys, cpu.registers.a);
        6
    };

    increment_program_counter(cpu, 4);
    cycles
}
