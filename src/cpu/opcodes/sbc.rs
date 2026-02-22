use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            calculate_indirect_page_address, calculate_indirect_page_x_address,
            calculate_indirect_page_y_address, get_carry_in, get_x_register_value,
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, page_crossed,
            read_data_byte, read_data_byte_indirect_y, read_data_word, read_data_word_indirect_y,
            read_offset_byte, read_offset_word, read_word_direct_page,
            set_nz_flags_u8, set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// SBC - Subtract with Carry (Borrow)
// Subtracts a value from memory and the inverse of the carry flag from the accumulator. Sets N, Z, C, and V flags.
// Used for multi-byte subtraction and arithmetic operations. Supports 8-bit and 16-bit modes.

pub fn sbc_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_subtraction_with_carry_u8(cpu, value as u16);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_subtraction_with_carry_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn sbc_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(source_address as u32) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        3
    } else {
        let value = read_word_direct_page(bus, source_address);
        perform_subtraction_with_carry_u16(cpu, value);
        4
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sbc_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sbc_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_subtraction_with_carry_u16(cpu, value);
        5
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sbc_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let index = get_x_register_value(cpu);
    let address = base_address.wrapping_add(index);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (index as u32)) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        4
    } else {
        let value = bus.read_word( phys);
        perform_subtraction_with_carry_u16(cpu, value);
        5
    };

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sbc_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address.wrapping_add(cpu.registers.y);
    let phys =
        (((cpu.registers.db as u32) << 16) + (base_address as u32) + (cpu.registers.y as u32))
            & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(phys) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        4
    } else {
        let value = bus.read_word( phys);
        perform_subtraction_with_carry_u16(cpu, value);
        5
    };

    if !is_8bit_mode_x(cpu) || page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn sbc_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address) = calculate_indirect_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);
        7
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sbc_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address16) = calculate_indirect_page_y_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let (value, extra) = read_data_byte_indirect_y(cpu, bus, base_address, address16);
        perform_subtraction_with_carry_u8(cpu, value as u16);
        5 + (extra as u8)
    } else {
        let (value, extra) = read_data_word_indirect_y(cpu, bus, base_address, address16);
        perform_subtraction_with_carry_u16(cpu, value);
        6 + (extra as u8)
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn sbc_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address) as u16;
        perform_subtraction_with_carry_u8(cpu, value);
        5
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_subtraction_with_carry_u16(cpu, value);
        6
    };

    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

// SBC in decimal mode is implemented as ADC with one's-complemented operand.
// A - M - (1-C) is algebraically identical to A + ~M + C.
// The BCD correction for subtraction mirrors ADC but subtracts 6 when a nibble
// does NOT carry (underflow) instead of adding 6 when it overflows.
//
// Flag behavior on the 65C816 in decimal mode:
//   N, Z  — from the final BCD-corrected result
//   C     — from the BCD computation (carry out = no borrow)
//   V     — from the intermediate result (after low nibble fix, before high nibble fix)

fn perform_subtraction_with_carry_u8(cpu: &mut Cpu, value: u16) {
    let old_accumulator_full = cpu.registers.a;
    let a = (old_accumulator_full & 0x00FF) as i32;
    let v = (value & 0x00FF) as i32;
    let carry_in = get_carry_in(cpu) as i32;

    let decimal = cpu.registers.p.contains(ProcessorStatus::DECIMAL);

    if decimal {
        let complement = v ^ 0xFF;

        // Low nibble: A + ~M + C (using signed arithmetic to handle underflow)
        let mut r: i32 = (a & 0x0F) + (complement & 0x0F) + carry_in;
        if r <= 0x0F {
            r -= 0x06;
        }
        let lo_carry: i32 = if r > 0x0F { 0x10 } else { 0x00 };

        // High nibble + carry from low + corrected low nibble
        r = (a & 0xF0) + (complement & 0xF0) + lo_carry + (r & 0x0F);

        // V from intermediate (after low nibble fix, before high nibble fix)
        let intermediate = (r & 0xFF) as u16;
        let a_u16 = a as u16;
        let v_u16 = v as u16;
        cpu.registers.p.set(
            ProcessorStatus::OVERFLOW,
            ((a_u16 ^ intermediate) & (a_u16 ^ v_u16) & 0x80) != 0,
        );

        if r <= 0xFF {
            r -= 0x60;
        }

        let final_result = (r & 0xFF) as u8;
        cpu.registers.a = (old_accumulator_full & 0xFF00) | (final_result as u16);
        set_nz_flags_u8(cpu, final_result);
        cpu.registers.p.set(ProcessorStatus::CARRY, r > 0xFF);
    } else {
        let diff = a - v - (1 - carry_in);
        let result8 = (diff as u16) & 0x00FF;

        cpu.registers.a = (old_accumulator_full & 0xFF00) | result8;
        set_nz_flags_u8(cpu, result8 as u8);
        cpu.registers.p.set(ProcessorStatus::CARRY, diff >= 0);
        cpu.registers.p.set(
            ProcessorStatus::OVERFLOW,
            (((a as u16) ^ result8) & ((a as u16) ^ (v as u16)) & 0x80) != 0,
        );
    }
}

fn perform_subtraction_with_carry_u16(cpu: &mut Cpu, value: u16) {
    let old_accumulator = cpu.registers.a;
    let carry_in = get_carry_in(cpu) as i32;

    let decimal = cpu.registers.p.contains(ProcessorStatus::DECIMAL);

    if decimal {
        let a = old_accumulator as i32;
        let v = value as i32;
        let complement = v ^ 0xFFFF;

        // Nibble 0 (bits 0-3)
        let mut r: i32 = (a & 0x000F) + (complement & 0x000F) + carry_in;
        if r <= 0x000F {
            r -= 0x0006;
        }
        let n0_carry: i32 = if r > 0x000F { 0x0010 } else { 0x0000 };

        // Nibble 1 (bits 4-7)
        r = (a & 0x00F0) + (complement & 0x00F0) + n0_carry + (r & 0x000F);
        if r <= 0x00FF {
            r -= 0x0060;
        }
        let n1_carry: i32 = if r > 0x00FF { 0x0100 } else { 0x0000 };

        // Nibble 2 (bits 8-11)
        r = (a & 0x0F00) + (complement & 0x0F00) + n1_carry + (r & 0x00FF);
        if r <= 0x0FFF {
            r -= 0x0600;
        }
        let n2_carry: i32 = if r > 0x0FFF { 0x1000 } else { 0x0000 };

        // Nibble 3 (bits 12-15)
        r = (a & 0xF000) + (complement & 0xF000) + n2_carry + (r & 0x0FFF);

        // V from intermediate (before final nibble correction)
        let intermediate = (r & 0xFFFF) as u16;
        cpu.registers.p.set(
            ProcessorStatus::OVERFLOW,
            ((old_accumulator ^ intermediate) & (old_accumulator ^ value) & 0x8000) != 0,
        );

        if r <= 0xFFFF {
            r -= 0x6000;
        }

        let final_result = (r & 0xFFFF) as u16;
        cpu.registers.a = final_result;
        set_nz_flags_u16(cpu, final_result);
        cpu.registers.p.set(ProcessorStatus::CARRY, r > 0xFFFF);
    } else {
        let result = (old_accumulator as i32) - (value as i32) - (1 - carry_in);
        let result_u16 = result as u16;

        cpu.registers.a = result_u16;
        set_nz_flags_u16(cpu, result_u16);
        cpu.registers.p.set(ProcessorStatus::CARRY, result >= 0);
        cpu.registers.p.set(
            ProcessorStatus::OVERFLOW,
            ((old_accumulator ^ result_u16) & (old_accumulator ^ value) & 0x8000) != 0,
        );
    }
}
