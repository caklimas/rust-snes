use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            increment_program_counter, is_8bit_mode_x, read_program_byte, write_data_byte,
            write_data_word,
        },
    },
    memory::MemoryBus,
};

// STY - Store Y Register
// Stores the Y register value to memory. Does not affect any processor flags.

// STY (0x84) - Direct Page
pub fn sty_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_x(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.y as u8);
        3
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.y);
        4
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// STY (0x8C) - Absolute
pub fn sty_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
    let target_address = (address_high as u16) << 8 | (address_low as u16);

    let cycles = if is_8bit_mode_x(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.y as u8);
        4
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.y);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

// STY (0x94) - Direct Page Indexed by X
pub fn sty_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_x(cpu) {
        write_data_byte(cpu, bus, address, cpu.registers.y as u8);
        4
    } else {
        write_data_word(cpu, bus, address, cpu.registers.y);
        5
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}
