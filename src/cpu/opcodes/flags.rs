use crate::{
    cpu::{
        Cpu,
        opcodes::{increment_program_counter, read_offset_byte},
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// SEC - Set Carry Flag
// Sets the carry flag to 1. Commonly used before subtraction operations or to indicate success/true conditions.
pub fn sec<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.insert(ProcessorStatus::CARRY);
    increment_program_counter(cpu, 1);
    2
}

// CLC - Clear Carry Flag
// Clears the carry flag to 0. Commonly used before addition operations or to indicate failure/false conditions.
pub fn clc<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.remove(ProcessorStatus::CARRY);
    increment_program_counter(cpu, 1);
    2
}

// SEI - Set Interrupt Disable Flag
// Sets the interrupt disable flag, preventing maskable interrupts (IRQ) from being processed.
// Non-maskable interrupts (NMI) can still occur.
pub fn sei<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.insert(ProcessorStatus::IRQ_DISABLE);
    increment_program_counter(cpu, 1);
    2
}

// CLI - Clear Interrupt Disable Flag
// Clears the interrupt disable flag, allowing maskable interrupts (IRQ) to be processed.
pub fn cli<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.remove(ProcessorStatus::IRQ_DISABLE);
    increment_program_counter(cpu, 1);
    2
}

// SED - Set Decimal Mode Flag
// Sets the decimal mode flag. In decimal mode, ADC and SBC operate on BCD (Binary Coded Decimal) values.
// Note: The 65816 decimal mode is not fully implemented in all operations.
pub fn sed<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.insert(ProcessorStatus::DECIMAL);
    increment_program_counter(cpu, 1);
    2
}

// CLD - Clear Decimal Mode Flag
// Clears the decimal mode flag. ADC and SBC operate on binary values (normal mode).
pub fn cld<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.remove(ProcessorStatus::DECIMAL);
    increment_program_counter(cpu, 1);
    2
}

// CLV - Clear Overflow Flag
// Clears the overflow flag to 0. The overflow flag indicates signed arithmetic overflow.
pub fn clv<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.p.remove(ProcessorStatus::OVERFLOW);
    increment_program_counter(cpu, 1);
    2
}

// SEP - Set Processor Status Bits
// Sets processor status bits according to the immediate operand.
// Each bit in the operand that is 1 will set the corresponding flag.
// 65816-specific instruction commonly used to switch between 8-bit and 16-bit modes.
pub fn sep<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let mask = read_offset_byte(cpu, bus);
    let bits_to_set = ProcessorStatus::from_bits_truncate(mask);
    cpu.registers.p.insert(bits_to_set);

    increment_program_counter(cpu, 2);
    3
}

// REP - Reset Processor Status Bits
// Clears processor status bits according to the immediate operand.
// Each bit in the operand that is 1 will clear the corresponding flag.
// 65816-specific instruction commonly used to switch between 8-bit and 16-bit modes.
pub fn rep<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let mask = read_offset_byte(cpu, bus);
    let bits_to_clear = ProcessorStatus::from_bits_truncate(mask);
    cpu.registers.p.remove(bits_to_clear);

    if cpu.emulation_mode {
        cpu.registers.p.insert(ProcessorStatus::MEMORY_WIDTH | ProcessorStatus::INDEX_WIDTH);
    }

    increment_program_counter(cpu, 2);
    3
}
