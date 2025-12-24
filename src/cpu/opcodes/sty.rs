use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address, get_x_register_value,
            increment_program_counter, is_8bit_mode_x, read_byte, read_offset_byte, write_byte,
            write_word,
        },
    },
    memory::MemoryBus,
};

// STY - Store Y Register
// Stores the Y register value to memory. Does not affect any processor flags.

// STY (0x84) - Direct Page
pub fn sty_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_x(cpu) {
        write_byte(cpu, bus, target_address, cpu.registers.y as u8);
        3
    } else {
        write_word(cpu, bus, target_address, cpu.registers.y);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

// STY (0x8C) - Absolute
pub fn sty_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address_low = read_byte(cpu, bus, cpu.registers.pc + 1);
    let address_high = read_byte(cpu, bus, cpu.registers.pc + 2);
    let target_address = (address_high as u16) << 8 | (address_low as u16);

    let cycles = if is_8bit_mode_x(cpu) {
        write_byte(cpu, bus, target_address, cpu.registers.y as u8);
        4
    } else {
        write_word(cpu, bus, target_address, cpu.registers.y);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

// STY (0x94) - Direct Page Indexed by X
pub fn sty_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_x(cpu) {
        write_byte(cpu, bus, address, cpu.registers.y as u8);
        4
    } else {
        write_word(cpu, bus, address, cpu.registers.y);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}
