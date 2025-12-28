use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, set_nz_flags_u8,
            set_nz_flags_u16,
        },
    },
    memory::{MemoryBus, addresses::STACK_START},
};

// TAX (0xAA) - Transfer Accumulator to X
// Copies the accumulator value into the X register, useful for setting up loop counters or array indices from calculated values.
pub fn tax<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        cpu.registers.x = (cpu.registers.x & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.x = cpu.registers.a;
        set_nz_flags_u16(cpu, cpu.registers.x);
    }

    increment_program_counter(cpu, 1);
    2
}

// TAY (0xA8) - Transfer Accumulator to Y
// Copies the accumulator value into the Y register, commonly used to set up the Y index register for indexed addressing modes.
pub fn tay<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        cpu.registers.y = (cpu.registers.y & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.y = cpu.registers.a;
        set_nz_flags_u16(cpu, cpu.registers.y);
    }

    increment_program_counter(cpu, 1);
    2
}

// TXA (0x8A) - Transfer X to Accumulator
// Copies the X register into the accumulator, allowing you to perform arithmetic or comparisons on index values.
pub fn txa<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.x & 0xFF) as u8;
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.a = cpu.registers.x;
        set_nz_flags_u16(cpu, cpu.registers.a);
    }

    increment_program_counter(cpu, 1);
    2
}

// TYA (0x98) - Transfer Y to Accumulator
// Copies the Y register into the accumulator, similar to TXA but for the Y register.
pub fn tya<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.y & 0xFF) as u8;
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.a = cpu.registers.y;
        set_nz_flags_u16(cpu, cpu.registers.a);
    }

    increment_program_counter(cpu, 1);
    2
}

// TSX (0xBA) - Transfer Stack Pointer to X
// Copies the stack pointer into X, typically used for stack manipulation or saving the current stack position.
pub fn tsx<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.s & 0xFF) as u8;
        cpu.registers.x = (cpu.registers.x & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.x = cpu.registers.s;
        set_nz_flags_u16(cpu, cpu.registers.x);
    }

    increment_program_counter(cpu, 1);
    2
}

// TXS (0x9A) - Transfer X to Stack Pointer (does NOT set flags)
// Sets the stack pointer to the value in X, used to restore a saved stack position or initialize the stack to a specific location.
pub fn txs<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.s = cpu.registers.x;

    increment_program_counter(cpu, 1);
    2
}

// TXY (0x9B) - Transfer X to Y (65816 only)
// Copies X register to Y register directly without going through the accumulator, useful for duplicating index values.
pub fn txy<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.x & 0xFF) as u8;
        cpu.registers.y = (cpu.registers.y & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.y = cpu.registers.x;
        set_nz_flags_u16(cpu, cpu.registers.y);
    }

    increment_program_counter(cpu, 1);
    2
}

// TYX (0xBB) - Transfer Y to X (65816 only)
// Copies Y register to X register directly, the reverse of TXY.
pub fn tyx<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = (cpu.registers.y & 0xFF) as u8;
        cpu.registers.x = (cpu.registers.x & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
    } else {
        cpu.registers.x = cpu.registers.y;
        set_nz_flags_u16(cpu, cpu.registers.x);
    }

    increment_program_counter(cpu, 1);
    2
}

// TCD (0x5B) - Transfer 16-bit Accumulator to Direct Page (65816 only)
// Sets the Direct Page register to the accumulator value, allowing you to change which page of memory the direct addressing mode points to.
pub fn tcd<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.d = cpu.registers.a;
    set_nz_flags_u16(cpu, cpu.registers.d);

    increment_program_counter(cpu, 1);
    2
}

// TDC (0x7B) - Transfer Direct Page to 16-bit Accumulator (65816 only)
// Copies the Direct Page register into the accumulator, typically used to save or examine the current direct page setting.
pub fn tdc<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.a = cpu.registers.d;
    set_nz_flags_u16(cpu, cpu.registers.a);

    increment_program_counter(cpu, 1);
    2
}

// TCS (0x1B) - Transfer 16-bit Accumulator to Stack Pointer (65816 only, does NOT set flags)
// Sets the stack pointer to the full 16-bit accumulator value, used for stack initialization or switching between multiple stacks.
pub fn tcs<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    if cpu.emulation_mode {
        cpu.registers.s = (STACK_START as u16) | (cpu.registers.a & 0x00FF);
    } else {
        cpu.registers.s = cpu.registers.a;
    }

    increment_program_counter(cpu, 1);
    2
}

// TSC (0x3B) - Transfer Stack Pointer to 16-bit Accumulator (65816 only)
// Copies the full 16-bit stack pointer into the accumulator, useful for calculating stack usage or implementing stack frames.
pub fn tsc<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    cpu.registers.a = cpu.registers.s;
    set_nz_flags_u16(cpu, cpu.registers.a);

    increment_program_counter(cpu, 1);
    2
}
