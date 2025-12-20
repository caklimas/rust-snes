use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, pull_byte, push_byte,
            read_offset_byte, read_offset_word, read_word, set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::bus::Bus,
};

// Stack Operations
// The 65816 stack grows downward in memory. Push operations decrement the stack pointer,
// pull operations increment it. Most pull operations set N and Z flags.

// PHA (0x48) - Push Accumulator
// Pushes the accumulator onto the stack. Pushes 1 byte in 8-bit mode, 2 bytes in 16-bit mode.
pub fn pha(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    if is_8bit_mode_m(cpu) {
        push_byte(cpu, bus, cpu.registers.a as u8);
        increment_program_counter(cpu, 1);
        3
    } else {
        push_byte(cpu, bus, (cpu.registers.a >> 8) as u8);
        push_byte(cpu, bus, cpu.registers.a as u8);
        increment_program_counter(cpu, 1);
        4
    }
}

// PHX (0xDA) - Push X Register
// Pushes the X register onto the stack. Pushes 1 byte in 8-bit mode, 2 bytes in 16-bit mode.
pub fn phx(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    if is_8bit_mode_x(cpu) {
        push_byte(cpu, bus, cpu.registers.x as u8);
        increment_program_counter(cpu, 1);
        3
    } else {
        push_byte(cpu, bus, (cpu.registers.x >> 8) as u8);
        push_byte(cpu, bus, cpu.registers.x as u8);
        increment_program_counter(cpu, 1);
        4
    }
}

// PHY (0x5A) - Push Y Register
// Pushes the Y register onto the stack. Pushes 1 byte in 8-bit mode, 2 bytes in 16-bit mode.
pub fn phy(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    if is_8bit_mode_x(cpu) {
        push_byte(cpu, bus, cpu.registers.y as u8);
        increment_program_counter(cpu, 1);
        3
    } else {
        push_byte(cpu, bus, (cpu.registers.y >> 8) as u8);
        push_byte(cpu, bus, cpu.registers.y as u8);
        increment_program_counter(cpu, 1);
        4
    }
}

// PHP (0x08) - Push Processor Status
// Pushes the processor status flags onto the stack. Always pushes 1 byte.
pub fn php(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    push_byte(cpu, bus, cpu.registers.p.bits());
    increment_program_counter(cpu, 1);
    3
}

// PHB (0x8B) - Push Data Bank Register (65816 only)
// Pushes the Data Bank register onto the stack. Always pushes 1 byte.
pub fn phb(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    push_byte(cpu, bus, cpu.registers.db);
    increment_program_counter(cpu, 1);
    3
}

// PHD (0x0B) - Push Direct Page Register (65816 only)
// Pushes the Direct Page register onto the stack. Always pushes 2 bytes.
pub fn phd(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    push_byte(cpu, bus, (cpu.registers.d >> 8) as u8);
    push_byte(cpu, bus, cpu.registers.d as u8);
    increment_program_counter(cpu, 1);
    5
}

// PHK (0x4B) - Push Program Bank Register (65816 only)
// Pushes the Program Bank register onto the stack. Always pushes 1 byte.
pub fn phk(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    push_byte(cpu, bus, cpu.registers.pb);
    increment_program_counter(cpu, 1);
    3
}

// PLA (0x68) - Pull Accumulator
// Pulls a value from the stack into the accumulator. Sets N and Z flags.
pub fn pla(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    if is_8bit_mode_m(cpu) {
        let value = pull_byte(cpu, bus);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
        increment_program_counter(cpu, 1);
        4
    } else {
        let low = pull_byte(cpu, bus);
        let high = pull_byte(cpu, bus);
        let value = ((high as u16) << 8) | (low as u16);
        cpu.registers.a = value;
        set_nz_flags_u16(cpu, value);
        increment_program_counter(cpu, 1);
        5
    }
}

// PLX (0xFA) - Pull X Register
// Pulls a value from the stack into the X register. Sets N and Z flags.
pub fn plx(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = pull_byte(cpu, bus);
        cpu.registers.x = (cpu.registers.x & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
        increment_program_counter(cpu, 1);
        4
    } else {
        let low = pull_byte(cpu, bus);
        let high = pull_byte(cpu, bus);
        let value = ((high as u16) << 8) | (low as u16);
        cpu.registers.x = value;
        set_nz_flags_u16(cpu, value);
        increment_program_counter(cpu, 1);
        5
    }
}

// PLY (0x7A) - Pull Y Register
// Pulls a value from the stack into the Y register. Sets N and Z flags.
pub fn ply(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    if is_8bit_mode_x(cpu) {
        let value = pull_byte(cpu, bus);
        cpu.registers.y = (cpu.registers.y & 0xFF00) | (value as u16);
        set_nz_flags_u8(cpu, value);
        increment_program_counter(cpu, 1);
        4
    } else {
        let low = pull_byte(cpu, bus);
        let high = pull_byte(cpu, bus);
        let value = ((high as u16) << 8) | (low as u16);
        cpu.registers.y = value;
        set_nz_flags_u16(cpu, value);
        increment_program_counter(cpu, 1);
        5
    }
}

// PLP (0x28) - Pull Processor Status
// Pulls the processor status flags from the stack. Does not set any flags (it restores them).
pub fn plp(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let value = pull_byte(cpu, bus);
    cpu.registers.p = crate::cpu::processor_status::ProcessorStatus::from_bits_truncate(value);
    increment_program_counter(cpu, 1);
    4
}

// PLB (0xAB) - Pull Data Bank Register (65816 only)
// Pulls a value from the stack into the Data Bank register. Sets N and Z flags.
pub fn plb(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let value = pull_byte(cpu, bus);
    cpu.registers.db = value;
    set_nz_flags_u8(cpu, value);
    increment_program_counter(cpu, 1);
    4
}

// PLD (0x2B) - Pull Direct Page Register (65816 only)
// Pulls a 16-bit value from the stack into the Direct Page register. Sets N and Z flags.
pub fn pld(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let low = pull_byte(cpu, bus);
    let high = pull_byte(cpu, bus);
    let value = ((high as u16) << 8) | (low as u16);
    cpu.registers.d = value;
    set_nz_flags_u16(cpu, value);
    increment_program_counter(cpu, 1);
    5
}

// PEA (0xF4) - Push Effective Absolute Address (65816 only)
// Pushes a 16-bit immediate value onto the stack. Used for passing parameters or creating stack frames.
pub fn pea(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let value = read_offset_word(cpu, bus);
    push_byte(cpu, bus, (value >> 8) as u8);
    push_byte(cpu, bus, value as u8);
    increment_program_counter(cpu, 3);
    5
}

// PEI (0xD4) - Push Effective Indirect Address (65816 only)
// Pushes the 16-bit value stored at (Direct Page + offset) onto the stack.
pub fn pei(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;
    let value = read_word(cpu, bus, address);
    push_byte(cpu, bus, (value >> 8) as u8);
    push_byte(cpu, bus, value as u8);
    increment_program_counter(cpu, 2);
    6
}

// PER (0x62) - Push Effective PC-Relative Address (65816 only)
// Pushes the result of (PC + 3 + signed offset) onto the stack. Used for position-independent code.
pub fn per(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_word(cpu, bus) as i16;
    let target_address = (cpu.registers.pc + 3).wrapping_add(offset as u16);
    push_byte(cpu, bus, (target_address >> 8) as u8);
    push_byte(cpu, bus, target_address as u8);
    increment_program_counter(cpu, 3);
    6
}
