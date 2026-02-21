use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_y_address,
            increment_program_counter, is_8bit_mode_x, page_crossed, read_offset_byte,
            read_offset_word, read_program_byte, read_program_word, read_word_direct_page,
            set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::MemoryBus,
};

// LDX - Load X Register
// Loads a value from memory into the X index register. Sets N and Z flags based on the loaded value.
// Supports 8-bit and 16-bit modes depending on the X flag.

// LDX (0xA2) - Immediate
pub fn ldx_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_x(cpu) {
        let value = read_offset_byte(cpu, bus);
        cpu.registers.x = (cpu.registers.x & 0xFF00) | value as u16;
        set_nz_flags_u8(cpu, value);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

// LDX (0xA6) - Direct Page
pub fn ldx_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_address(cpu, bus);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = bus.read(source_address as u32);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word_direct_page(bus, source_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDX (0xAE) - Absolute
pub fn ldx_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let absolute_address = read_offset_word(cpu, bus);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_program_byte(cpu, bus, absolute_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_program_word(cpu, bus, absolute_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 3);

    cycles
}

// LDX (0xB6) - Direct Page Indexed by Y
pub fn ldx_direct_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_y_address(cpu, bus);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_program_byte(cpu, bus, source_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_program_word(cpu, bus, source_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDX (0xBE) - Absolute Indexed by Y
pub fn ldx_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let target_address = base_address + cpu.registers.y;
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_program_byte(cpu, bus, target_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_program_word(cpu, bus, target_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if page_crossed(base_address, target_address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}
