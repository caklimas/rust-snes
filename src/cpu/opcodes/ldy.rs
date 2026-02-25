use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address, get_x_register_value,
            increment_program_counter, is_8bit_mode_x, page_crossed, read_data_byte,
            read_data_word, read_offset_byte, read_offset_word,
            read_word_direct_page, set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::MemoryBus,
};

// LDY - Load Y Register
// Loads a value from memory into the Y index register. Sets N and Z flags based on the loaded value.
// Supports 8-bit and 16-bit modes depending on the X flag.

// LDY (0xA0) - Immediate
pub fn ldy_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_x(cpu) {
        let value = read_offset_byte(cpu, bus); // u8
        cpu.registers.y = (cpu.registers.y & 0xFF00) | value as u16;
        set_nz_flags_u8(cpu, value);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus); // u16
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

// LDY (0xA4) - Direct Page
pub fn ldy_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_address(cpu, bus);
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = bus.read(source_address as u32);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word_direct_page(bus, source_address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDY (0xAC) - Absolute
pub fn ldy_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let absolute_address = read_offset_word(cpu, bus);
    let cycles;

    if is_8bit_mode_x(cpu) {
        let value = read_data_byte(cpu, bus, absolute_address);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_data_word(cpu, bus, absolute_address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

// LDY (0xB4) - Direct Page Indexed by X
pub fn ldy_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = bus.read(address as u32);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word_direct_page(bus, address);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);

    cycles
}

// LDY (0xBC) - Absolute Indexed by X
pub fn ldy_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let index = get_x_register_value(cpu);
    let target_address = base_address.wrapping_add(index);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;
    let mut cycles;

    if is_8bit_mode_x(cpu) {
        let value = bus.read(phys);
        cpu.registers.y = value as u16;
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = bus.read_word( phys);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, target_address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}
