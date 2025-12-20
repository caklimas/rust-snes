use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x,
            set_nz_flags_u8, set_nz_flags_u16,
        },
    },
    memory::bus::Bus,
};

// TAX - Transfer Accumulator to X
pub fn tax(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TAY - Transfer Accumulator to Y
pub fn tay(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TXA - Transfer X to Accumulator
pub fn txa(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TYA - Transfer Y to Accumulator
pub fn tya(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TSX - Transfer Stack Pointer to X
pub fn tsx(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TXS - Transfer X to Stack Pointer (does NOT set flags)
pub fn txs(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.s = cpu.registers.x;

    increment_program_counter(cpu, 1);
    2
}

// TXY - Transfer X to Y (65816 only)
pub fn txy(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TYX - Transfer Y to X (65816 only)
pub fn tyx(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
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

// TCD - Transfer 16-bit Accumulator to Direct Page (65816 only)
pub fn tcd(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.d = cpu.registers.a;
    set_nz_flags_u16(cpu, cpu.registers.d);

    increment_program_counter(cpu, 1);
    2
}

// TDC - Transfer Direct Page to 16-bit Accumulator (65816 only)
pub fn tdc(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.a = cpu.registers.d;
    set_nz_flags_u16(cpu, cpu.registers.a);

    increment_program_counter(cpu, 1);
    2
}

// TCS - Transfer 16-bit Accumulator to Stack Pointer (65816 only, does NOT set flags)
pub fn tcs(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.s = cpu.registers.a;

    increment_program_counter(cpu, 1);
    2
}

// TSC - Transfer Stack Pointer to 16-bit Accumulator (65816 only)
pub fn tsc(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    cpu.registers.a = cpu.registers.s;
    set_nz_flags_u16(cpu, cpu.registers.a);

    increment_program_counter(cpu, 1);
    2
}
