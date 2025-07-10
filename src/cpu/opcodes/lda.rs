use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_m, is_negative_u8, is_negative_u16,
            page_crossed, read_byte, read_offset_byte, read_word, set_nz_flags_u8,
            set_nz_flags_u16,
        },
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

pub fn lda_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let target_address = (cpu.registers.d + offset) as u32;
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 3;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn lda_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let base_address = cpu.registers.d + offset;
    let target_address = (base_address + cpu.registers.x) as u32;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    if page_crossed(target_address as u16, base_address) {
        cycles += 1;
    }

    cycles
}

pub fn lda_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut pc_increment = 2;
    let address = (cpu.registers.pc + 1) as u32;
    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
    } else {
        let value = read_word(bus, address);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);

        pc_increment += 1;
    }

    increment_program_counter(cpu, pc_increment);

    if is_8bit_mode_m(cpu) { 2 } else { 3 }
}

pub fn lda_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let target_address = ((address_high as u16) << 8 | (address_low as u16)).into();
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);

        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn lda_absolute_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let base_address = ((address_high as u16) << 8 | (address_low as u16)) as u16;
    let target_address = (base_address + cpu.registers.x) as u32;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    if page_crossed(target_address as u16, base_address) {
        cycles += 1;
    }

    cycles
}

pub fn lda_absolute_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(bus, (cpu.registers.pc + 2).into());
    let base_address = ((address_high as u16) << 8 | (address_low as u16)) as u16;
    let target_address = (base_address + cpu.registers.y) as u32;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 4;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    if page_crossed(target_address as u16, base_address) {
        cycles += 1;
    }

    cycles
}

pub fn lda_indirect(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = (cpu.registers.d + offset) as u32;

    let target_address_low = read_byte(bus, pointer_address) as u16;
    let target_address_high = read_byte(bus, pointer_address + 1) as u16;
    let target_address = ((target_address_high << 8) | target_address_low) as u32;
    let cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 5;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn lda_indirect_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let base_pointer_address = (cpu.registers.d + offset) as u32;
    let pointer_address = base_pointer_address + (cpu.registers.x as u32);

    let target_address_low = read_byte(bus, pointer_address) as u16;
    let target_address_high = read_byte(bus, pointer_address + 1) as u16;
    let target_address = ((target_address_high << 8) | target_address_low) as u32;
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 6;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 7;
    }

    increment_program_counter(cpu, 2);

    if page_crossed(base_pointer_address as u16, pointer_address as u16) {
        cycles += 1;
    }

    cycles
}

pub fn lda_indirect_y(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = (cpu.registers.d + offset) as u32;

    let base_address_low = read_byte(bus, pointer_address) as u16;
    let base_address_high = read_byte(bus, pointer_address + 1) as u16;
    let base_address = ((base_address_high << 8) | base_address_low) as u32;
    let target_address = base_address + (cpu.registers.y as u32);
    let mut cycles;

    if is_8bit_mode_m(cpu) {
        let value = read_byte(bus, target_address);
        set_accumulator_u8(cpu, value);
        set_nz_flags_u8(cpu, value);
        cycles = 5;
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        let value = get_value_u16(value_low, value_high);
        set_accumulator_u16(cpu, value);
        set_nz_flags_u16(cpu, value);
        cycles = 6;
    }

    increment_program_counter(cpu, 2);

    if page_crossed(base_address as u16, target_address as u16) {
        cycles += 1;
    }

    cycles
}

fn set_accumulator_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.a = (cpu.registers.a & 0xFF00) | (value as u16);
}

fn set_accumulator_u16(cpu: &mut Cpu, value: u16) {
    cpu.registers.a = value;
}

fn get_value_u16(value_low: u16, value_high: u16) -> u16 {
    (value_high << 8) | value_low
}
