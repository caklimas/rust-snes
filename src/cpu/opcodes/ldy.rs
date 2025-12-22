use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_x_register_value, increment_program_counter, is_8bit_mode_x, page_crossed,
            read_byte, read_offset_byte, read_offset_word, read_word, set_nz_flags_u8,
            set_nz_flags_u16,
        },
    },
    memory::MemoryBus,
};

// LDY - Load Y Register
// Loads a value from memory into the Y index register. Sets N and Z flags based on the loaded value.
// Supports 8-bit and 16-bit modes depending on the X flag.

// LDY (0xA0) - Immediate
pub fn ldy_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let cycles;
    let pc_increment;

    if is_8bit_mode_x(cpu) {
        let value = read_offset_byte(cpu, bus);
        cpu.registers.y = value;
        set_nz_flags_u8(cpu, value as u8);
        pc_increment = 2;
        cycles = 2;
    } else {
        let value = read_offset_word(cpu, bus);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        pc_increment = 3;
        cycles = 3;
    }

    increment_program_counter(cpu, pc_increment);

    cycles
}

// LDY (0xA4) - Direct Page
pub fn ldy_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let source_address = cpu.registers.d + offset;
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(cpu, bus, source_address);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word(cpu, bus, source_address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDY (0xAC) - Absolute
pub fn ldy_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let absolute_address = read_offset_word(cpu, bus);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(cpu, bus, absolute_address);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word(cpu, bus, absolute_address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 3);

    cycles
}

// LDY (0xB4) - Direct Page Indexed by X
pub fn ldy_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let source_address = cpu.registers.d + offset + get_x_register_value(cpu);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(cpu, bus, source_address);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(cpu, bus, source_address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDY (0xBC) - Absolute Indexed by X
pub fn ldy_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let target_address = base_address + get_x_register_value(cpu);
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_byte(cpu, bus, target_address);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(cpu, bus, target_address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if page_crossed(base_address, target_address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}
