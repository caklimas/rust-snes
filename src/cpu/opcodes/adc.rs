use crate::{
    cpu::{
        Cpu,
        opcodes::{
            helpers::{
                calculate_absolute_x_address, calculate_direct_page_address,
                calculate_direct_page_x_address, calculate_indirect_page_address,
                calculate_indirect_page_x_address, calculate_indirect_page_y_address, get_carry_in,
                increment_program_counter, is_8bit_mode_m, page_crossed, read_offset_byte,
                read_offset_word, read_program_byte, read_program_word, read_word_direct_page,
                set_nz_flags_u8, set_nz_flags_u16,
            },
            read_data_byte, read_data_word,
        },
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// ADC - Add with Carry
// Adds a value from memory to the accumulator plus the carry flag. Sets N, Z, C, and V flags.
// Used for multi-byte addition and arithmetic operations. Supports 8-bit and 16-bit modes.

pub fn adc_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_addition_with_carry_u8(cpu, value as u16);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_addition_with_carry_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn adc_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(source_address as u32) as u16;
        perform_addition_with_carry_u8(cpu, value);
        3
    } else {
        let value = read_program_word(cpu, bus, source_address);
        perform_addition_with_carry_u16(cpu, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn adc_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_program_byte(cpu, bus, address) as u16;
        perform_addition_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_program_word(cpu, bus, address);
        perform_addition_with_carry_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn adc_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32) as u16;
        perform_addition_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_addition_with_carry_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn adc_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = calculate_absolute_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_program_byte(cpu, bus, address) as u16;
        perform_addition_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_program_word(cpu, bus, address);
        perform_addition_with_carry_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn adc_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.y;

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_program_byte(cpu, bus, address) as u16;
        perform_addition_with_carry_u8(cpu, value);
        4
    } else {
        let value = read_program_word(cpu, bus, address);
        perform_addition_with_carry_u16(cpu, value);
        5
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn adc_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address16) = calculate_indirect_page_x_address(cpu, bus);

    // Base cycles for ADC (dp,X): 6 (M=1) / 7 (M=0)
    let mut cycles = if is_8bit_mode_m(cpu) { 6 } else { 7 };

    // Direct-page penalty when D low byte != 0
    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    if is_8bit_mode_m(cpu) {
        // IMPORTANT: data comes from DBR, not PBR
        let value = read_data_byte(cpu, bus, address16) as u16;
        perform_addition_with_carry_u8(cpu, value);
    } else {
        let value = read_data_word(cpu, bus, address16);
        perform_addition_with_carry_u16(cpu, value);
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn adc_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_address, address) = calculate_indirect_page_y_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_program_byte(cpu, bus, address) as u16;
        perform_addition_with_carry_u8(cpu, value);
        5
    } else {
        let value = read_program_word(cpu, bus, address);
        perform_addition_with_carry_u16(cpu, value);
        6
    };

    if page_crossed(base_address, address) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn adc_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_indirect_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_program_byte(cpu, bus, address) as u16;
        perform_addition_with_carry_u8(cpu, value);
        5
    } else {
        let value = read_program_word(cpu, bus, address);
        perform_addition_with_carry_u16(cpu, value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

fn perform_addition_with_carry_u8(cpu: &mut Cpu, value: u16) {
    let old_accumulator_full = cpu.registers.a;
    let a = old_accumulator_full & 0x00FF;

    let carry_in = get_carry_in(cpu) & 0x0001;
    let v = value & 0x00FF;

    let decimal = cpu.registers.p.contains(ProcessorStatus::DECIMAL);

    let (final_result8, carry_out) = if decimal {
        let mut lo = (a & 0x000F) + (v & 0x000F) + carry_in;
        let mut hi = (a & 0x00F0) + (v & 0x00F0);

        if lo > 0x0009 {
            lo += 0x0006;
        }
        if lo > 0x000F {
            hi += 0x0010;
        }

        // V flag is computed from the intermediate result: after low nibble
        // correction but before high nibble correction (matches 65C816 behavior)
        let intermediate = ((hi | (lo & 0x000F)) & 0x00FF) as u16;
        set_v_flag_u8(cpu, a, intermediate, v);

        if hi > 0x0090 {
            hi += 0x0060;
        }

        (((hi & 0x00F0) | (lo & 0x000F)) as u8, hi > 0x00FF)
    } else {
        let binary_sum = a + v + carry_in;
        let binary_result8 = (binary_sum & 0x00FF) as u8;
        set_v_flag_u8(cpu, a, binary_result8 as u16, v);
        (binary_result8, binary_sum > 0x00FF)
    };

    cpu.registers.a = (old_accumulator_full & 0xFF00) | (final_result8 as u16);

    set_nz_flags_u8(cpu, final_result8);
    cpu.registers.p.set(ProcessorStatus::CARRY, carry_out);
}

fn perform_addition_with_carry_u16(cpu: &mut Cpu, value: u16) {
    let old_accumulator = cpu.registers.a;
    let carry_in = get_carry_in(cpu);

    let decimal = cpu.registers.p.contains(ProcessorStatus::DECIMAL);

    if decimal {
        let a = old_accumulator as u32;
        let v = value as u32;

        // Nibble 0 (bits 0-3)
        let mut r = (a & 0x000F) + (v & 0x000F) + (carry_in as u32);
        if r > 0x0009 {
            r += 0x0006;
        }
        let n0_carry: u32 = if r > 0x000F { 0x0010 } else { 0x0000 };

        // Nibble 1 (bits 4-7)
        r = (a & 0x00F0) + (v & 0x00F0) + n0_carry + (r & 0x000F);
        if r > 0x009F {
            r += 0x0060;
        }
        let n1_carry: u32 = if r > 0x00FF { 0x0100 } else { 0x0000 };

        // Nibble 2 (bits 8-11)
        r = (a & 0x0F00) + (v & 0x0F00) + n1_carry + (r & 0x00FF);
        if r > 0x09FF {
            r += 0x0600;
        }
        let n2_carry: u32 = if r > 0x0FFF { 0x1000 } else { 0x0000 };

        // Nibble 3 (bits 12-15)
        r = (a & 0xF000) + (v & 0xF000) + n2_carry + (r & 0x0FFF);

        // V from intermediate (before final nibble correction)
        let intermediate = (r & 0xFFFF) as u16;
        set_v_flag_u16(cpu, old_accumulator, intermediate, value);

        if r > 0x9FFF {
            r += 0x6000;
        }

        let final_result = (r & 0xFFFF) as u16;
        cpu.registers.a = final_result;
        set_nz_flags_u16(cpu, final_result);
        cpu.registers.p.set(ProcessorStatus::CARRY, r > 0xFFFF);
    } else {
        let result = (old_accumulator as u32) + (value as u32) + (carry_in as u32);
        let result_u16 = result as u16;

        cpu.registers.a = result_u16;
        set_nz_flags_u16(cpu, result_u16);
        set_c_flag_u16(cpu, result);
        set_v_flag_u16(cpu, old_accumulator, result_u16, value);
    }
}

fn set_v_flag_u8(cpu: &mut Cpu, old_accumulator: u16, result: u16, value: u16) {
    let a = old_accumulator & 0x00FF;
    let r = result & 0x00FF;
    let v = value & 0x00FF;

    cpu.registers
        .p
        .set(ProcessorStatus::OVERFLOW, (!(a ^ v) & (a ^ r) & 0x80) != 0);
}

fn set_c_flag_u16(cpu: &mut Cpu, result: u32) {
    cpu.registers.p.set(ProcessorStatus::CARRY, result > 0xFFFF);
}

fn set_v_flag_u16(cpu: &mut Cpu, old_accumulator: u16, result: u16, value: u16) {
    cpu.registers.p.set(
        ProcessorStatus::OVERFLOW,
        (!(old_accumulator ^ value) & (old_accumulator ^ result) & 0x8000) != 0,
    );
}
