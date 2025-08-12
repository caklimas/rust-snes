use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_x_register_value, increment_program_counter, is_8bit_mode_x, read_byte,
            read_offset_byte, write_byte, write_word,
        },
    },
    memory::bus::Bus,
};

pub fn sty_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let target_address = (cpu.registers.d + offset) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        write_byte(bus, target_address, cpu.registers.y as u8 & 0xFF);
        cycles = 3;
    } else {
        write_word(bus, target_address, cpu.registers.y);
        cycles = 4;
    }

    increment_program_counter(cpu, 2);

    cycles
}

pub fn sty_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(cpu, bus, (cpu.registers.pc + 1).into());
    let address_high = read_byte(cpu, bus, (cpu.registers.pc + 2).into());
    let target_address = ((address_high as u16) << 8 | (address_low as u16)).into();
    let cycles;

    if is_8bit_mode_x(cpu) {
        write_byte(bus, target_address, (cpu.registers.y as u8) & 0xFF);
        cycles = 4;
    } else {
        write_word(bus, target_address, cpu.registers.y);
        cycles = 5;
    }

    increment_program_counter(cpu, 3);

    cycles
}

pub fn sty_direct_x(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let offset = read_offset_byte(cpu, bus);
    let target_address = (cpu.registers.d + offset + get_x_register_value(cpu)) as u32;
    let cycles;

    if is_8bit_mode_x(cpu) {
        write_byte(bus, target_address, (cpu.registers.y as u8) & 0xFF);
        cycles = 4;
    } else {
        write_word(bus, target_address, cpu.registers.y);
        cycles = 5;
    }

    increment_program_counter(cpu, 2);

    cycles
}
