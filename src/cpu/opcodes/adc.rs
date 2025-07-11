use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_carry_in, increment_program_counter, is_8bit_mode_m, read_byte, read_offset_byte,
            read_offset_word, read_word, set_nz_flags_u8, set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

pub fn adc_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let pc_increment;

    if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_addition_with_carry_u8(cpu, bus, value);

        pc_increment = 2;
        cycles = 2;
    } else {
        let value = read_offset_word(cpu, bus);
        perform_addition_with_carry_u16(cpu, bus, value);

        pc_increment = 3;
        cycles = 3;
    }

    increment_program_counter(cpu, pc_increment);

    cycles
}

pub fn adc_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;
    let offset = read_offset_byte(cpu, bus);
    let source_address = (cpu.registers.d + offset) as u32;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, source_address) as u16;
        perform_addition_with_carry_u8(cpu, bus, value);

        cycles = 3;
    } else {
        let value = read_word(bus, source_address);
        perform_addition_with_carry_u16(cpu, bus, value);

        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

fn perform_addition_with_carry_u8(cpu: &mut Cpu, bus: &mut Bus, value: u16) {
    let old_accumulator = cpu.registers.a;
    let carry_in = get_carry_in(cpu);
    let result = (old_accumulator & 0xFF) + value + carry_in;

    cpu.registers.a = (cpu.registers.a & 0xFF00) | (result & 0xFF);
    set_nz_flags_u8(cpu, (result & 0xFF) as u8);
    set_c_flag_u8(cpu, result);
    set_v_flag_u8(cpu, old_accumulator, result, value);
}

fn perform_addition_with_carry_u16(cpu: &mut Cpu, bus: &mut Bus, value: u16) {
    let old_accumulator = cpu.registers.a;
    let carry_in = get_carry_in(cpu);
    let result = (old_accumulator as u32) + (value as u32) + (carry_in as u32);
    let result_u16 = result as u16;

    cpu.registers.a = result_u16;
    set_nz_flags_u16(cpu, result_u16);
    set_c_flag_u16(cpu, result);
    set_v_flag_u16(cpu, old_accumulator, result_u16, value);
}

fn set_c_flag_u8(cpu: &mut Cpu, result: u16) {
    cpu.registers.p.set(ProcessorStatus::CARRY, result > 0xFF);
}

fn set_v_flag_u8(cpu: &mut Cpu, old_accumulator: u16, result: u16, value: u16) {
    cpu.registers.p.set(
        ProcessorStatus::OVERFLOW,
        ((old_accumulator ^ result) & (value ^ result) & 0x80) != 0,
    );
}

fn set_c_flag_u16(cpu: &mut Cpu, result: u32) {
    cpu.registers.p.set(ProcessorStatus::CARRY, result > 0xFFFF);
}

fn set_v_flag_u16(cpu: &mut Cpu, old_accumulator: u16, result: u16, value: u16) {
    cpu.registers.p.set(
        ProcessorStatus::OVERFLOW,
        ((old_accumulator ^ result) & (value ^ result) & 0x8000) != 0,
    );
}
