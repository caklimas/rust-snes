use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, get_x_register_value, read_byte_direct_page,
            read_offset_word, read_program_word, read_word_direct_page,
        },
    },
    memory::MemoryBus,
};

// JMP (0x4C) - Jump Absolute
// Unconditionally jumps to the specified 16-bit address within the current bank.
pub fn jmp_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    cpu.registers.pc = address;

    3
}

// JML (0x5C) - Jump Absolute Long
// Unconditionally jumps to a 24-bit address, allowing jumps across program banks.
pub fn jmp_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = calculate_absolute_long_address(cpu, bus);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    4
}

// JMP (0x6C) - Jump Absolute Indirect
// Jumps to the address stored at the specified memory location (pointer jump).
pub fn jmp_absolute_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let pointer_address = read_offset_word(cpu, bus);
    let address = read_word_direct_page(bus, pointer_address);

    cpu.registers.pc = address;

    5
}

// JMP (0x7C) - Jump Absolute Indexed Indirect
// Jumps to the address stored at (base pointer + X register), useful for jump tables.
pub fn jmp_absolute_indexed_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_pointer = read_offset_word(cpu, bus);
    let pointer_address = base_pointer + get_x_register_value(cpu);
    let address = read_program_word(cpu, bus, pointer_address);

    cpu.registers.pc = address;

    6
}

// JML (0xDC) - Jump Absolute Indirect Long
// Jumps to the 24-bit address stored at the specified memory location, allowing indirect jumps across banks.
pub fn jmp_absolute_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let pointer_address = read_offset_word(cpu, bus);
    let address_low = read_byte_direct_page(bus, pointer_address);
    let address_mid = read_byte_direct_page(bus, pointer_address.wrapping_add(1));
    let address_high = read_byte_direct_page(bus, pointer_address.wrapping_add(2));
    let target_address =
        (address_high as u32) << 16 | (address_mid as u32) << 8 | (address_low as u32);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    6
}
