use crate::{
    cpu::Cpu,
    memory::{MemoryBus, addresses::STACK_START},
};

use super::super::StackMode;

/// Push a byte onto the stack
pub(crate) fn push_byte<B: MemoryBus>(
    cpu: &mut Cpu,
    bus: &mut B,
    value: u8,
    stack_mode: StackMode,
) {
    let stack_address = get_stack_address(cpu, stack_mode);
    // Stack is always in bank 0
    bus.write(stack_address as u32, value);
    decrement_stack_pointer(cpu, stack_mode);
}

/// Pull a byte from the stack
pub(crate) fn pull_byte<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, stack_mode: StackMode) -> u8 {
    increment_stack_pointer(cpu, stack_mode);
    let stack_address = get_stack_address(cpu, stack_mode);
    // Stack is always in bank 0
    bus.read(stack_address as u32)
}

/// Get the current stack address based on the mode
pub(crate) fn get_stack_address(cpu: &Cpu, stack_mode: StackMode) -> u16 {
    if !cpu.emulation_mode {
        // Native mode: always linear 16-bit stack
        return cpu.registers.s;
    }

    // Emulation mode: choose behavior by StackMode
    match stack_mode {
        StackMode::EmuPage1 => (STACK_START as u16) | (cpu.registers.s & 0x00FF),
        StackMode::Linear16 => cpu.registers.s,
    }
}

/// Decrement the stack pointer
pub(crate) fn decrement_stack_pointer(cpu: &mut Cpu, stack_mode: StackMode) {
    if !cpu.emulation_mode {
        cpu.registers.s = cpu.registers.s.wrapping_sub(1);
        return;
    }

    match stack_mode {
        StackMode::EmuPage1 => {
            let low = (cpu.registers.s as u8).wrapping_sub(1);
            cpu.registers.s = (STACK_START as u16) | low as u16;
        }
        StackMode::Linear16 => {
            cpu.registers.s = cpu.registers.s.wrapping_sub(1);
        }
    }
}

/// Increment the stack pointer
pub(crate) fn increment_stack_pointer(cpu: &mut Cpu, stack_mode: StackMode) {
    if !cpu.emulation_mode {
        cpu.registers.s = cpu.registers.s.wrapping_add(1);
        return;
    }

    match stack_mode {
        StackMode::EmuPage1 => {
            let low = (cpu.registers.s as u8).wrapping_add(1);
            cpu.registers.s = 0x0100 | low as u16;
        }
        StackMode::Linear16 => {
            cpu.registers.s = cpu.registers.s.wrapping_add(1);
        }
    }
}

/**
 * SST compatibility: For certain "new" 65816 ops in E=1, SST treats stack accesses as linear
 * 16-bit during execution ($0100 -> $00FF), then forces final S back to $01xx afterward.
 * Normalize S here to match SST; not representative of real hardware stack addressing.
 * https://github.com/SingleStepTests/ProcessorTests/issues/44?utm_source=chatgpt.com
 */
pub(crate) fn normalize_stack_pointer(cpu: &mut Cpu) {
    if cpu.emulation_mode {
        cpu.registers.s = (STACK_START as u16) | (cpu.registers.s & 0x00FF);
    }
}
