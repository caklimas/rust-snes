use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_x_register_value, increment_program_counter, is_8bit_mode_m, read_byte,
            read_offset_byte, read_offset_word, read_word, set_nz_flags_u8, set_nz_flags_u16,
            write_byte, write_word,
        },
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

// ASL - Arithmetic Shift Left
// Shifts all bits left by one position. Bit 0 is filled with 0, and the original bit 7/15 goes into the carry flag.
// Sets N, Z, and C flags. Effectively multiplies the value by 2.

pub fn asl_accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn asl_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn asl_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn asl_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn asl_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let result = value << 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// LSR - Logical Shift Right
// Shifts all bits right by one position. Bit 7/15 is filled with 0, and the original bit 0 goes into the carry flag.
// Sets N (always 0), Z, and C flags. Effectively divides the value by 2 (unsigned).

pub fn lsr_accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn lsr_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn lsr_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn lsr_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn lsr_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let result = value >> 1;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// ROL - Rotate Left
// Shifts all bits left by one position. The carry flag goes into bit 0, and bit 7/15 goes into the carry flag.
// Sets N, Z, and C flags. Used for multi-byte shifts and bit manipulation.

pub fn rol_accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn rol_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn rol_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn rol_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn rol_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 1 } else { 0 };
        let result = (value << 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// ROR - Rotate Right
// Shifts all bits right by one position. The carry flag goes into bit 7/15, and bit 0 goes into the carry flag.
// Sets N, Z, and C flags. Used for multi-byte shifts and bit manipulation.

pub fn ror_accumulator(cpu: &mut Cpu, _bus: &mut Bus) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x8000 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn ror_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x8000 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ror_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x8000 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ror_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x8000 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ror_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x80 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) { 0x8000 } else { 0 };
        let result = (value >> 1) | carry_in;
        cpu.registers.p.set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}
