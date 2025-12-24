use crate::{
    cpu::{
        Cpu,
        opcodes::{
            StackMode, get_address_absolute_long, normalize_stack_pointer, push_byte,
            read_offset_word,
        },
    },
    memory::MemoryBus,
};

// JSR (0x20) - Jump to Subroutine Absolute
// Pushes the return address onto the stack and jumps to the specified 16-bit address, used for calling subroutines.
pub fn jsr_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, stack_mode: StackMode) -> u8 {
    let address = read_offset_word(cpu, bus);
    let return_address = cpu.registers.pc + 2;

    push_word(cpu, bus, return_address, stack_mode);

    cpu.registers.pc = address;

    6
}

// JSL (0x22) - Jump to Subroutine Long
// Pushes the program bank and return address onto the stack, then jumps to a 24-bit address for cross-bank subroutine calls.
pub fn jsr_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let target_address = get_address_absolute_long(cpu, bus);
    let return_address = cpu.registers.pc + 3;

    push_byte(cpu, bus, cpu.registers.pb, StackMode::Linear16);
    push_word(cpu, bus, return_address, StackMode::Linear16);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    normalize_stack_pointer(cpu);

    8
}

fn push_word<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, address: u16, stack_mode: StackMode) {
    push_byte(cpu, bus, ((address >> 8) & 0xFF) as u8, stack_mode);
    push_byte(cpu, bus, (address & 0xFF) as u8, stack_mode);
}
