use crate::{
    cpu::{Cpu, processor_status::ProcessorStatus},
    memory::bus::Bus,
};

use super::pull_byte;

// RTS - Return from Subroutine
// Returns from a subroutine called by JSR. Pulls the return address from the stack and increments it by 1.
// Does not affect any flags. The return address pushed by JSR is the address of the last byte of the JSR instruction,
// so RTS adds 1 to get the next instruction.
pub fn rts(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let return_address_low = pull_byte(cpu, bus) as u16;
    let return_address_high = pull_byte(cpu, bus) as u16;
    let return_address = (return_address_high << 8) | return_address_low;

    // JSR pushes PC+2 (the last byte of JSR instruction), so we add 1 to get next instruction
    cpu.registers.pc = return_address.wrapping_add(1);

    6
}

// RTL - Return from Subroutine Long
// Returns from a long subroutine called by JSR Long. Pulls a 24-bit return address (including bank byte) from the stack.
// Does not affect any flags. Used for returning from subroutines that cross bank boundaries.
pub fn rtl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let return_address_low = pull_byte(cpu, bus) as u16;
    let return_address_high = pull_byte(cpu, bus) as u16;
    let return_bank = pull_byte(cpu, bus);
    let return_address = (return_address_high << 8) | return_address_low;

    // JSL pushes PC+3 (the last byte of JSL instruction), so we add 1 to get next instruction
    cpu.registers.pc = return_address.wrapping_add(1);
    cpu.registers.pb = return_bank;

    6
}

// RTI - Return from Interrupt
// Returns from an interrupt handler. Pulls the processor status and return address from the stack.
// Restores all processor flags and the program counter to the state before the interrupt occurred.
// In native mode (E=0), also restores the program bank register.
pub fn rti(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    // Pull processor status
    let status_byte = pull_byte(cpu, bus);
    cpu.registers.p = ProcessorStatus::from_bits_truncate(status_byte);

    if cpu.emulation_mode {
        // Emulation mode: pull 16-bit address only
        let return_address_low = pull_byte(cpu, bus) as u16;
        let return_address_high = pull_byte(cpu, bus) as u16;
        cpu.registers.pc = (return_address_high << 8) | return_address_low;
        6
    } else {
        // Native mode: pull 24-bit address (PC + PB)
        let return_address_low = pull_byte(cpu, bus) as u16;
        let return_address_high = pull_byte(cpu, bus) as u16;
        let return_bank = pull_byte(cpu, bus);
        cpu.registers.pc = (return_address_high << 8) | return_address_low;
        cpu.registers.pb = return_bank;
        7
    }
}
