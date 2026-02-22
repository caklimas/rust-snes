use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_y_address,
            increment_program_counter, is_8bit_mode_x, read_program_byte, write_data_byte,
            write_data_word,
        },
    },
    memory::MemoryBus,
};

// STX - Store X Register
// Stores the X register value to memory. Does not affect any processor flags.

// STX (0x86) - Direct Page
pub fn stx_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_x(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.x as u8);
        3
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.x);
        4
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// STX (0x96) - Direct Page Indexed by Y
pub fn stx_direct_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_y_address(cpu, bus);
    let mut cycles = if is_8bit_mode_x(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.x as u8);
        4
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.x);
        5
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// STX (0x8E) - Absolute
pub fn stx_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_program_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_program_byte(cpu, bus, cpu.registers.pc + 2);
    let target_address = (address_high as u16) << 8 | (address_low as u16);

    let cycles = if is_8bit_mode_x(cpu) {
        write_data_byte(cpu, bus, target_address, cpu.registers.x as u8);
        4
    } else {
        write_data_word(cpu, bus, target_address, cpu.registers.x);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}
