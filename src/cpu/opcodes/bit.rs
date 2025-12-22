use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_m, read_byte, read_offset_byte,
            read_offset_word, read_word, write_byte, write_word,
        },
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

// BIT - Bit Test
// Tests bits in memory against the accumulator using AND operation.
// Sets Z flag if (A AND M) == 0. In non-immediate modes, also copies bit 7 of M to N flag and bit 6 of M to V flag.
// Does not modify the accumulator or memory.
pub fn bit_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus) as u8;
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn bit_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        perform_bit_test_u8(cpu, a_value, value);
        3
    } else {
        let value = read_word(cpu, bus, address);
        perform_bit_test_u16(cpu, cpu.registers.a, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn bit_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        perform_bit_test_u8(cpu, a_value, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_bit_test_u16(cpu, cpu.registers.a, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn bit_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let x_value = if cpu.registers.p.contains(ProcessorStatus::INDEX_WIDTH) {
        cpu.registers.x & 0xFF
    } else {
        cpu.registers.x
    };
    let address = cpu.registers.d + offset + x_value;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        perform_bit_test_u8(cpu, a_value, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_bit_test_u16(cpu, cpu.registers.a, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn bit_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.x;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        perform_bit_test_u8(cpu, a_value, value);
        4
    } else {
        let value = read_word(cpu, bus, address);
        perform_bit_test_u16(cpu, cpu.registers.a, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

fn perform_bit_test_u8(cpu: &mut Cpu, a_value: u8, value: u8) {
    let result = a_value & value;
    cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, value & 0x80 != 0);
    cpu.registers
        .p
        .set(ProcessorStatus::OVERFLOW, value & 0x40 != 0);
}

fn perform_bit_test_u16(cpu: &mut Cpu, a_value: u16, value: u16) {
    let result = a_value & value;
    cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, value & 0x8000 != 0);
    cpu.registers
        .p
        .set(ProcessorStatus::OVERFLOW, value & 0x4000 != 0);
}

// TSB - Test and Set Bits
// Tests bits in memory against accumulator (sets Z flag if (A AND M) == 0).
// Then sets bits in memory: M = M OR A. Modifies memory, not accumulator.
pub fn tsb_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | a_value;
        write_byte(cpu, bus, address, new_value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | cpu.registers.a;
        write_word(cpu, bus, address, new_value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn tsb_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | a_value;
        write_byte(cpu, bus, address, new_value);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | cpu.registers.a;
        write_word(cpu, bus, address, new_value);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

// TRB - Test and Reset Bits
// Tests bits in memory against accumulator (sets Z flag if (A AND M) == 0).
// Then clears bits in memory: M = M AND (NOT A). Modifies memory, not accumulator.
pub fn trb_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let address = cpu.registers.d + offset;

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !a_value;
        write_byte(cpu, bus, address, new_value);
        5
    } else {
        let value = read_word(cpu, bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !cpu.registers.a;
        write_word(cpu, bus, address, new_value);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn trb_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !a_value;
        write_byte(cpu, bus, address, new_value);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !cpu.registers.a;
        write_word(cpu, bus, address, new_value);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}
