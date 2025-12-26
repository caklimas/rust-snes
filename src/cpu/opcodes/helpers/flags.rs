use crate::cpu::{processor_status::ProcessorStatus, Cpu};

/// Check if CPU is in 8-bit memory mode (M flag set)
pub(crate) fn is_8bit_mode_m(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::MEMORY_WIDTH)
}

/// Check if CPU is in 8-bit index mode (X flag set)
pub(crate) fn is_8bit_mode_x(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::INDEX_WIDTH)
}

/// Check if direct page low byte is zero
pub(crate) fn direct_page_low_is_zero(cpu: &Cpu) -> bool {
    cpu.registers.d & 0x00FF == 0
}

/// Increment the program counter by the specified value
pub(crate) fn increment_program_counter(cpu: &mut Cpu, value: u16) {
    cpu.registers.pc = cpu.registers.pc.wrapping_add(value);
}

/// Set the negative and zero flags based on an 8-bit value
pub(crate) fn set_nz_flags_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, is_negative_u8(value));
}

/// Set the negative and zero flags based on a 16-bit value
pub(crate) fn set_nz_flags_u16(cpu: &mut Cpu, value: u16) {
    cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, is_negative_u16(value));
}

/// Check if an 8-bit value is negative (bit 7 set)
fn is_negative_u8(value: u8) -> bool {
    value & 0x80 != 0
}

/// Check if a 16-bit value is negative (bit 15 set)
fn is_negative_u16(value: u16) -> bool {
    value & 0x8000 != 0
}

/// Get the carry flag value as a u16
pub(crate) fn get_carry_in(cpu: &Cpu) -> u16 {
    if cpu.registers.p.contains(ProcessorStatus::CARRY) {
        1
    } else {
        0
    }
}
