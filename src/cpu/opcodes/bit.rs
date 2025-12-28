use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            direct_page_low_is_zero, increment_program_counter, is_8bit_mode_m, read_byte,
            read_data_byte, read_data_word, read_offset_byte, read_offset_word, read_word,
            read_word_direct_page, write_byte, write_byte_direct_page, write_word,
            write_word_direct_page,
        },
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// BIT - Bit Test
// Tests bits in memory against the accumulator using AND operation.
// Sets Z flag if (A AND M) == 0. In non-immediate modes, also copies bit 7 of M to N flag and bit 6 of M to V flag.
// Does not modify the accumulator or memory.
pub fn bit_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
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

pub fn bit_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        perform_bit_test_u8(cpu, a_value, value);
        3
    } else {
        let value = read_word_direct_page(bus, address);
        perform_bit_test_u16(cpu, cpu.registers.a, value);
        4
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn bit_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
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

pub fn bit_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        perform_bit_test_u8(cpu, a_value, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_bit_test_u16(cpu, cpu.registers.a, value);
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn bit_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
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
pub fn tsb_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | a_value;
        write_byte_direct_page(bus, address, new_value);
        5
    } else {
        let value = read_word_direct_page(bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | cpu.registers.a;
        write_word_direct_page(bus, address, new_value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn tsb_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | a_value;
        write_byte(cpu, bus, address, new_value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value | cpu.registers.a;
        write_word(cpu, bus, address, new_value);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// TRB - Test and Reset Bits
// Tests bits in memory against accumulator (sets Z flag if (A AND M) == 0).
// Then clears bits in memory: M = M AND (NOT A). Modifies memory, not accumulator.
pub fn trb_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !a_value;
        write_byte_direct_page(bus, address, new_value);
        5
    } else {
        let value = read_word_direct_page(bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !cpu.registers.a;
        write_word_direct_page(bus, address, new_value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn trb_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        let a_value = (cpu.registers.a & 0xFF) as u8;
        let result = a_value & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !a_value;
        write_byte(cpu, bus, address, new_value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        let result = cpu.registers.a & value;
        cpu.registers.p.set(ProcessorStatus::ZERO, result == 0);
        let new_value = value & !cpu.registers.a;
        write_word(cpu, bus, address, new_value);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}
