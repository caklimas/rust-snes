use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            calculate_indirect_page_address, calculate_indirect_page_x_address,
            calculate_indirect_page_y_address, effective_phys_indirect_y, get_x_register_value,
            increment_program_counter, is_8bit_mode_m, read_program_byte, write_data_byte,
            write_data_word,
        },
    },
    memory::MemoryBus,
};

// STA - Store Accumulator
// Stores the accumulator value to memory. Does not affect any processor flags.

pub fn sta_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        3
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
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
        write_data_byte(cpu, bus, target_address, cpu.registers.a as u8);
        4
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.a);
        5
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sta_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
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
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
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
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
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
    let (_, _, pointer_address) = calculate_indirect_page_x_address(cpu, bus);
    let target_address_low = read_program_byte(cpu, bus, pointer_address) as u16;
    let target_address_high = read_program_byte(cpu, bus, pointer_address + 1) as u16;
    let target_address = (target_address_high << 8) | target_address_low;

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
