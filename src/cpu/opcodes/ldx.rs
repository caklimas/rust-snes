use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_y_address,
            increment_program_counter, is_8bit_mode_x, page_crossed, read_data_byte,
            read_data_word, read_offset_byte, read_offset_word,
            read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
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
    let mut cycles;

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

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDX (0xAE) - Absolute
pub fn ldx_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let absolute_address = read_offset_word(cpu, bus);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_data_byte(cpu, bus, absolute_address);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_data_word(cpu, bus, absolute_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

// LDX (0xB6) - Direct Page Indexed by Y
pub fn ldx_direct_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_y_address(cpu, bus);
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = bus.read(source_address as u32);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word_direct_page(bus, source_address);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDX (0xBE) - Absolute Indexed by Y
pub fn ldx_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let target_address = base_address.wrapping_add(cpu.registers.y);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (cpu.registers.y as u32))
            & 0x00FF_FFFF;
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = bus.read(phys);
        cpu.registers.x = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = bus.read_word( phys);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, target_address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}
