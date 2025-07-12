use crate::{
    cpu::{
        Cpu,
        opcodes::{is_8bit_mode_m, read_offset_byte, set_nz_flags_u8, set_nz_flags_u16},
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

pub fn cmp_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        let accumulator_value = cpu.registers.a & 0xFF;
        let result = accumulator_value - value;

        perform_compare_with_carry_u8(cpu, result, accumulator_value, value);

        cycles = 2;
    } else {
        cycles = 3;
    }

    cycles
}

fn perform_compare_with_carry_u8(cpu: &mut Cpu, result: u16, accumulator_value: u16, value: u16) {
    set_nz_flags_u8(cpu, (result & 0xFF) as u8);
    set_c_flag_u8(cpu, accumulator_value, value);
}

fn set_c_flag_u8(cpu: &mut Cpu, accumulator_value: u16, value: u16) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, accumulator_value > value);
}

fn perform_compare_with_carry_u16(cpu: &mut Cpu, result: u32, accumulator_value: u32, value: u32) {
    let result_u16 = result as u16;

    set_nz_flags_u16(cpu, result_u16);
    set_c_flag_u16(cpu, accumulator_value, value);
}

fn set_c_flag_u16(cpu: &mut Cpu, accumulator_value: u32, value: u32) {
    cpu.registers
        .p
        .set(ProcessorStatus::CARRY, accumulator_value > value);
}
