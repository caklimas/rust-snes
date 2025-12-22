use crate::{
    cpu::{
        Cpu,
        opcodes::{increment_program_counter, push_byte, set_nz_flags_u8},
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// NOP - No Operation
// Does nothing except consume CPU cycles. Often used for timing delays or as placeholder instructions.
pub fn nop<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    increment_program_counter(cpu, 1);
    2
}

// XBA - Exchange B and A
// Swaps the high byte (B) and low byte (A) of the 16-bit accumulator register.
// Sets N and Z flags based on the new low byte (A) value.
// This is useful for accessing both bytes of a 16-bit value in 8-bit mode.
pub fn xba<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let low_byte = (cpu.registers.a & 0xFF) as u8;
    let high_byte = ((cpu.registers.a >> 8) & 0xFF) as u8;

    cpu.registers.a = ((low_byte as u16) << 8) | (high_byte as u16);

    set_nz_flags_u8(cpu, high_byte);
    increment_program_counter(cpu, 1);
    3
}

// XCE - Exchange Carry and Emulation flags
// Swaps the carry flag with the emulation mode flag.
// Used to switch between 6502 emulation mode and native 65816 mode.
// When switching to emulation mode (E=1), forces M=1 and X=1 (8-bit modes) and clears high byte of X and Y.
pub fn xce<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let old_carry = cpu.registers.p.contains(ProcessorStatus::CARRY);
    let old_emulation = cpu.emulation_mode;

    cpu.registers.p.set(ProcessorStatus::CARRY, old_emulation);
    cpu.emulation_mode = old_carry;

    // When entering emulation mode, set M and X flags and clear high bytes
    if cpu.emulation_mode {
        cpu.registers.p.insert(ProcessorStatus::MEMORY_WIDTH);
        cpu.registers.p.insert(ProcessorStatus::INDEX_WIDTH);
        cpu.registers.x &= 0xFF;
        cpu.registers.y &= 0xFF;
    }

    increment_program_counter(cpu, 1);
    2
}

// BRK - Break
// Triggers a software interrupt. Pushes PC+2, then processor status to stack, then jumps to interrupt vector.
// Sets the interrupt disable flag. In emulation mode, also sets the break flag.
// The interrupt vector is at $FFFE-$FFFF in emulation mode, $FFE6-$FFE7 in native mode.
pub fn brk<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // Push return address (PC + 2)
    let return_address = cpu.registers.pc.wrapping_add(2);
    push_byte(cpu, bus, ((return_address >> 8) & 0xFF) as u8);
    push_byte(cpu, bus, (return_address & 0xFF) as u8);

    // Push processor status
    if !cpu.emulation_mode {
        // In native mode, push the program bank first
        push_byte(cpu, bus, cpu.registers.pb);
    }

    let mut status_byte = cpu.registers.p.bits();
    if cpu.emulation_mode {
        // In emulation mode, set bit 4 (B flag) in the pushed status byte
        status_byte |= 0x10;
    }
    push_byte(cpu, bus, status_byte);

    // Set interrupt disable flag
    cpu.registers.p.insert(ProcessorStatus::IRQ_DISABLE);

    // Clear decimal mode (happens on 65816 but not original 6502)
    cpu.registers.p.remove(ProcessorStatus::DECIMAL);

    // Jump to BRK vector
    let vector_address = if cpu.emulation_mode { 0xFFFE } else { 0xFFE6 };

    let new_pc_low = bus.read(vector_address as u32);
    let new_pc_high = bus.read((vector_address + 1) as u32);
    cpu.registers.pc = ((new_pc_high as u16) << 8) | (new_pc_low as u16);

    // Set PB to 0 (interrupt vectors are always in bank 0)
    cpu.registers.pb = 0;

    if cpu.emulation_mode { 7 } else { 8 }
}

// COP - Coprocessor
// Similar to BRK but uses a different interrupt vector ($FFF4-$FFF5 in emulation, $FFE4-$FFE5 in native).
// Originally intended for coprocessor support, but commonly used as a second software interrupt.
pub fn cop<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // Push return address (PC + 2)
    let return_address = cpu.registers.pc.wrapping_add(2);
    push_byte(cpu, bus, ((return_address >> 8) & 0xFF) as u8);
    push_byte(cpu, bus, (return_address & 0xFF) as u8);

    // In native mode, push program bank
    if !cpu.emulation_mode {
        push_byte(cpu, bus, cpu.registers.pb);
    }

    // Push processor status
    push_byte(cpu, bus, cpu.registers.p.bits());

    // Set interrupt disable flag
    cpu.registers.p.insert(ProcessorStatus::IRQ_DISABLE);

    // Clear decimal mode
    cpu.registers.p.remove(ProcessorStatus::DECIMAL);

    // Jump to COP vector
    let vector_address = if cpu.emulation_mode { 0xFFF4 } else { 0xFFE4 };

    let new_pc_low = bus.read(vector_address as u32);
    let new_pc_high = bus.read((vector_address + 1) as u32);
    cpu.registers.pc = ((new_pc_high as u16) << 8) | (new_pc_low as u16);

    // Set PB to 0 (interrupt vectors are always in bank 0)
    cpu.registers.pb = 0;

    if cpu.emulation_mode { 7 } else { 8 }
}

// WAI - Wait for Interrupt
// Puts the CPU into a low-power state until an interrupt (IRQ or NMI) occurs.
// The CPU stops executing instructions but continues to monitor interrupt lines.
pub fn wai<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.waiting_for_interrupt = true;
    increment_program_counter(cpu, 1);
    3
}

// STP - Stop the Processor
// Halts the processor completely until a hardware reset occurs.
// The CPU stops executing and does not respond to interrupts.
// Only a reset can restart execution.
pub fn stp<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.stopped = true;
    increment_program_counter(cpu, 1);
    3
}

// WDM - Reserved
// Reserved for future expansion. Currently acts like a 2-byte NOP.
pub fn wdm<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    increment_program_counter(cpu, 2);
    2
}
