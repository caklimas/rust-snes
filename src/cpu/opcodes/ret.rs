use crate::{
    cpu::{
        Cpu,
        opcodes::{StackMode, normalize_stack_pointer},
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

use super::pull_byte;

// RTS - Return from Subroutine
// Returns from a subroutine called by JSR. Pulls the return address from the stack and increments it by 1.
// Does not affect any flags. The return address pushed by JSR is the address of the last byte of the JSR instruction,
// so RTS adds 1 to get the next instruction.
pub fn rts<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, stack_mode: StackMode) -> u8 {
    let return_address_low = pull_byte(cpu, bus, stack_mode) as u16;
    let return_address_high = pull_byte(cpu, bus, stack_mode) as u16;
    let return_address = (return_address_high << 8) | return_address_low;

    // JSR pushes PC+2 (the last byte of JSR instruction), so we add 1 to get next instruction
    cpu.registers.pc = return_address.wrapping_add(1);

    6
}

// RTL - Return from Subroutine Long
// Returns from a long subroutine called by JSR Long. Pulls a 24-bit return address (including bank byte) from the stack.
// Does not affect any flags. Used for returning from subroutines that cross bank boundaries.
pub fn rtl<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let return_address_low = pull_byte(cpu, bus, StackMode::Linear16) as u16;
    let return_address_high = pull_byte(cpu, bus, StackMode::Linear16) as u16;
    let return_bank = pull_byte(cpu, bus, StackMode::Linear16);
    let return_address = (return_address_high << 8) | return_address_low;

    // JSL pushes PC+3 (the last byte of JSL instruction), so we add 1 to get next instruction
    cpu.registers.pc = return_address.wrapping_add(1);
    cpu.registers.pb = return_bank;

    normalize_stack_pointer(cpu);

    6
}

pub fn rti<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, stack_mode: StackMode) -> u8 {
    let status_byte = pull_byte(cpu, bus, stack_mode);
    let mut p = ProcessorStatus::from_bits_truncate(status_byte);

    if cpu.emulation_mode {
        p.insert(ProcessorStatus::MEMORY_WIDTH);
        p.insert(ProcessorStatus::INDEX_WIDTH);
    }

    cpu.registers.p = p;

    if cpu.registers.p.contains(ProcessorStatus::INDEX_WIDTH) {
        cpu.registers.x &= 0x00FF;
        cpu.registers.y &= 0x00FF;
    }

    if cpu.emulation_mode {
        let return_address_low = pull_byte(cpu, bus, stack_mode) as u16;
        let return_address_high = pull_byte(cpu, bus, stack_mode) as u16;
        cpu.registers.pc = (return_address_high << 8) | return_address_low;
        6
    } else {
        let return_address_low = pull_byte(cpu, bus, stack_mode) as u16;
        let return_address_high = pull_byte(cpu, bus, stack_mode) as u16;
        let return_bank = pull_byte(cpu, bus, stack_mode);
        cpu.registers.pc = (return_address_high << 8) | return_address_low;
        cpu.registers.pb = return_bank;
        7
    }
}
