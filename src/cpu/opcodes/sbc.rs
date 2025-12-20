use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_carry_in, get_x_register_value, increment_program_counter, is_8bit_mode_m,
            page_crossed, read_byte, read_offset_byte, read_offset_word, read_word,
            set_nz_flags_u8, set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

// SBC - Subtract with Carry (Borrow)
// Subtracts a value from memory and the inverse of the carry flag from the accumulator. Sets N, Z, C, and V flags.
// Used for multi-byte subtraction and arithmetic operations. Supports 8-bit and 16-bit modes.

pub fn sbc_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let pc_increment;

    if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_subtraction_with_carry_u8(cpu, value);

        pc_increment = 2;
        cycles = 2;
    } else {
        let value = read_offset_word(cpu, bus);
        perform_subtraction_with_carry_u16(cpu, value);

        pc_increment = 3;
        cycles = 3;
    }

    increment_program_counter(cpu, pc_increment);

    cycles
}

pub fn sbc_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let source_address = cpu.registers.d + offset;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, source_address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 3;
    } else {
        let value = read_word(cpu, bus, source_address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sbc_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let address = read_offset_word(cpu, bus);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sbc_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sbc_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut cycles;
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 5;
    }

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sbc_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut cycles;
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.y;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 4;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 5;
    }

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sbc_indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset + get_x_register_value(cpu);
    let address = read_word(cpu, bus, pointer_address);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 6;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 7;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sbc_indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut cycles;
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    let base_address = read_word(cpu, bus, pointer_address);
    let address = base_address + cpu.registers.y;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 5;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 6;
    }

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sbc_indirect(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    let address = read_word(cpu, bus, pointer_address);

    if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);

        cycles = 5;
    } else {
        let value = read_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);

        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    cycles
}

fn perform_subtraction_with_carry_u8(cpu: &mut Cpu, value: u16) {
    let old_accumulator = cpu.registers.a;
    let carry_in = get_carry_in(cpu) as i16;
    let result = (old_accumulator & 0xFF) as i16 - (value as i16) - (1 - carry_in);

    cpu.registers.a = (cpu.registers.a & 0xFF00) | (result & 0xFF) as u16;
    set_nz_flags_u8(cpu, (result & 0xFF) as u8);
    set_c_flag_u8(cpu, result);
    set_v_flag_u8(cpu, old_accumulator, result as u16, value);
}

fn perform_subtraction_with_carry_u16(cpu: &mut Cpu, value: u16) {
    let old_accumulator = cpu.registers.a;
    let carry_in = get_carry_in(cpu);
    let result = (old_accumulator as i32) - (value as i32) - (1 - carry_in as i32);
    let result_u16 = result as u16;

    cpu.registers.a = result_u16;
    set_nz_flags_u16(cpu, result_u16);
    set_c_flag_u16(cpu, result);
    set_v_flag_u16(cpu, old_accumulator, result_u16, value);
}

fn set_c_flag_u8(cpu: &mut Cpu, result: i16) {
    cpu.registers.p.set(ProcessorStatus::CARRY, result >= 0);
}

fn set_v_flag_u8(cpu: &mut Cpu, old_accumulator: u16, result: u16, value: u16) {
    cpu.registers.p.set(
        ProcessorStatus::OVERFLOW,
        ((old_accumulator ^ value) & (old_accumulator ^ result) & 0x80) != 0,
    );
}

fn set_c_flag_u16(cpu: &mut Cpu, result: i32) {
    cpu.registers.p.set(ProcessorStatus::CARRY, result >= 0);
}

fn set_v_flag_u16(cpu: &mut Cpu, old_accumulator: u16, result: u16, value: u16) {
    cpu.registers.p.set(
        ProcessorStatus::OVERFLOW,
        ((old_accumulator ^ value) & (old_accumulator ^ result) & 0x8000) != 0,
    );
}
