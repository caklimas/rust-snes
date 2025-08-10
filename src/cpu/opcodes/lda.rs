use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_address_absolute_x, increment_program_counter, is_8bit_mode_m, page_crossed,
            read_byte, read_offset_byte, read_offset_word, read_word, set_nz_flags_u8,
            set_nz_flags_u16,
        },
    },
    memory::bus::Bus,
};

pub fn lda_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let target_address = cpu.registers.d + offset;
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn lda_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let base_address = cpu.registers.d + offset;
    let target_address = base_address + cpu.registers.x;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    if page_crossed(target_address as u16, base_address) {
        cycles += 1;
    }

    cycles
}

pub fn lda_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut pc_increment = 2;
    let address = cpu.registers.pc + 1;
    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
    } else {
        let value = read_word(cpu, bus, address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);

        pc_increment += 1;
    }

    increment_program_counter(cpu, pc_increment);

    if is_8bit_mode_m(cpu) { 2 } else { 3 }
}

pub fn lda_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let target_address = read_offset_word(cpu, bus);
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn lda_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (base_address, target_address) = get_address_absolute_x(cpu, bus);
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    if page_crossed(base_address, target_address) {
        cycles += 1;
    }

    cycles
}

pub fn lda_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offet = read_offset_word(cpu, bus);
    let target_address = offet + cpu.registers.y;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    if page_crossed(offet as u16, target_address) {
        cycles += 1;
    }

    cycles
}

pub fn lda_indirect(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;

    let target_address = read_word(cpu, bus, pointer_address);
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 5;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn lda_indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let base_pointer_address = cpu.registers.d + offset;
    let pointer_address = base_pointer_address + cpu.registers.x;

    let target_address = read_word(cpu, bus, pointer_address);
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 6;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 7;
    }

    increment_program_counter(cpu, 2);

    if page_crossed(base_pointer_address as u16, pointer_address as u16) {
        cycles += 1;
    }

    cycles
}

pub fn lda_indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;

    let base_address = read_word(cpu, bus, pointer_address);
    let target_address = base_address + cpu.registers.y;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 5;
    } else {
        let value = read_word(cpu, bus, target_address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    if page_crossed(base_address as u16, target_address as u16) {
        cycles += 1;
    }

    cycles
}

fn set_accumulator_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
}

fn set_accumulator_u16(cpu: &mut Cpu, value: u16) {
    cpu.registers.a = value;
}
