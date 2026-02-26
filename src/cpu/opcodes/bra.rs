use crate::{
    cpu::{
        Cpu,
        opcodes::{read_offset_byte, read_offset_word},
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// BRA (0x80) - Branch Always
// Unconditionally branches to a relative offset, commonly used for short forward or backward jumps.
pub fn bra_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let displacement = read_offset_byte(cpu, bus) as i8;
    let pc_after = cpu.registers.pc.wrapping_add(2);
    let target = pc_after.wrapping_add(displacement as u16);
    let page_crossed = (pc_after & 0xFF00) != (target & 0xFF00);

    cpu.registers.pc = target;

    if cpu.emulation_mode && page_crossed {
        4
    } else {
        3
    }
}

// BRL (0x82) - Branch Always Long
// Unconditionally branches using a 16-bit relative offset, allowing longer jumps within the same bank.
pub fn bra_relative_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let displacement = read_offset_word(cpu, bus) as i16;
    let target_address = (cpu.registers.pc + 3).wrapping_add(displacement as u16);

    cpu.registers.pc = target_address;

    4
}

// BEQ (0xF0) - Branch if Equal (Zero flag set)
// Branches if the last operation resulted in zero (Z flag = 1), commonly used after comparisons.
pub fn beq<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(cpu, bus, cpu.registers.p.contains(ProcessorStatus::ZERO))
}

// BNE (0xD0) - Branch if Not Equal (Zero flag clear)
// Branches if the last operation did not result in zero (Z flag = 0), often used in loops.
pub fn bne<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(cpu, bus, !cpu.registers.p.contains(ProcessorStatus::ZERO))
}

// BCC (0x90) - Branch if Carry Clear
// Branches if the carry flag is clear (C flag = 0), used after additions or comparisons for unsigned less-than checks.
pub fn bcc<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(cpu, bus, !cpu.registers.p.contains(ProcessorStatus::CARRY))
}

// BCS (0xB0) - Branch if Carry Set
// Branches if the carry flag is set (C flag = 1), used after additions or comparisons for unsigned greater-than-or-equal checks.
pub fn bcs<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(cpu, bus, cpu.registers.p.contains(ProcessorStatus::CARRY))
}

// BMI (0x30) - Branch if Minus (Negative flag set)
// Branches if the negative flag is set (N flag = 1), indicating the last result was negative in signed arithmetic.
pub fn bmi<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(
        cpu,
        bus,
        cpu.registers.p.contains(ProcessorStatus::NEGATIVE),
    )
}

// BPL (0x10) - Branch if Plus (Negative flag clear)
// Branches if the negative flag is clear (N flag = 0), indicating the last result was positive or zero in signed arithmetic.
pub fn bpl<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(
        cpu,
        bus,
        !cpu.registers.p.contains(ProcessorStatus::NEGATIVE),
    )
}

// BVC (0x50) - Branch if Overflow Clear
// Branches if the overflow flag is clear (V flag = 0), used to check for valid signed arithmetic results.
pub fn bvc<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(
        cpu,
        bus,
        !cpu.registers.p.contains(ProcessorStatus::OVERFLOW),
    )
}

// BVS (0x70) - Branch if Overflow Set
// Branches if the overflow flag is set (V flag = 1), indicating signed arithmetic overflow occurred.
pub fn bvs<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    branch_conditional(
        cpu,
        bus,
        cpu.registers.p.contains(ProcessorStatus::OVERFLOW),
    )
}

fn branch_conditional<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, take_branch: bool) -> u8 {
    let pc_after = cpu.registers.pc.wrapping_add(2);

    if !take_branch {
        cpu.registers.pc = pc_after;
        return 2;
    }

    let displacement = read_offset_byte(cpu, bus) as i8;
    let target = pc_after.wrapping_add(displacement as u16);

    let mut cycles = 3; // base 2 + 1 if taken

    // Emulation mode behaves like 6502: +1 more if page boundary crossed
    if cpu.emulation_mode && ((pc_after & 0xFF00) != (target & 0xFF00)) {
        cycles += 1;
    }

    cpu.registers.pc = target;
    cycles
}
