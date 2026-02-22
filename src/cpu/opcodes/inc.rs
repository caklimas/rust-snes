use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address, get_x_register_value,
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, read_data_byte,
            read_data_word, read_offset_word, read_word, read_word_direct_page,
            set_nz_flags_u8, set_nz_flags_u16, write_data_byte, write_data_word, write_word,
        },
    },
    memory::MemoryBus,
};

// INC - Increment Memory
// Adds 1 to the value at a memory location. Sets N and Z flags based on the result.
// Commonly used for incrementing counters, loop indices, and other memory-based values.

pub fn inc_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let result = value.wrapping_add(1);
        write_data_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word_direct_page(bus, address);
        let result = value.wrapping_add(1);
        write_data_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn inc_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        let result = value.wrapping_add(1);
        write_data_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        let result = value.wrapping_add(1);
        write_data_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn inc_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let result = value.wrapping_add(1);
        write_data_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word_direct_page(bus, address);
        let result = value.wrapping_add(1);
        write_data_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn inc_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let index = get_x_register_value(cpu);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys);
        let result = value.wrapping_add(1);
        bus.write(phys, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(bus, phys);
        let result = value.wrapping_add(1);
        write_word(bus, phys, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// INA - Increment Accumulator
// Adds 1 to the accumulator. Sets N and Z flags based on the result.
// Commonly used for simple accumulator-based counting.
pub fn ina<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let result = value.wrapping_add(1);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let result = cpu.registers.a.wrapping_add(1);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

// INX - Increment X Register
// Adds 1 to the X register. Sets N and Z flags based on the result.
// Commonly used in loops and array indexing.
pub fn inx<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.x & 0xFF) as u8;
        let result = value.wrapping_add(1);
        cpu.registers.x = (cpu.registers.x & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let result = cpu.registers.x.wrapping_add(1);
        cpu.registers.x = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

// INY - Increment Y Register
// Adds 1 to the Y register. Sets N and Z flags based on the result.
// Commonly used in loops and array indexing.
pub fn iny<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.y & 0xFF) as u8;
        let result = value.wrapping_add(1);
        cpu.registers.y = (cpu.registers.y & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let result = cpu.registers.y.wrapping_add(1);
        cpu.registers.y = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}
