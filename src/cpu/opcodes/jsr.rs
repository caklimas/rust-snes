use crate::{
    cpu::{
        Cpu,
        opcodes::{
            StackMode, calculate_absolute_long_address, get_x_register_value,
            normalize_stack_pointer, push_byte, read_offset_word, read_program_word,
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
    let target_address = calculate_absolute_long_address(cpu, bus);
    let return_address = cpu.registers.pc + 3;

    push_byte(cpu, bus, cpu.registers.pb, StackMode::Linear16);
    push_word(cpu, bus, return_address, StackMode::Linear16);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    normalize_stack_pointer(cpu);

    8
}

// JSR (0xFC) - Jump to Subroutine Absolute Indexed Indirect: (abs,X)
// Pushes the return address (PC+2), then jumps to the address read from PBR:(operand+X).
pub fn jsr_absolute_indexed_indirect<B: MemoryBus>(
    cpu: &mut Cpu,
    bus: &mut B,
    stack_mode: StackMode,
) -> u8 {
    let return_address = cpu.registers.pc.wrapping_add(2);
    let base_pointer = read_offset_word(cpu, bus);
    let pointer_address = base_pointer.wrapping_add(get_x_register_value(cpu));

    push_word(cpu, bus, return_address, stack_mode);

    let target = read_program_word(cpu, bus, pointer_address);
    cpu.registers.pc = target;

    8
}

fn push_word<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, address: u16, stack_mode: StackMode) {
    push_byte(cpu, bus, ((address >> 8) & 0xFF) as u8, stack_mode);
    push_byte(cpu, bus, (address & 0xFF) as u8, stack_mode);
}
