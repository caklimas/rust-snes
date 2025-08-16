use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_address_absolute_long, get_x_register_value, read_byte, read_offset_word, read_word,
        },
    },
    memory::bus::Bus,
};

pub fn jmp_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);

    cpu.registers.pc = address;

    3
}

pub fn jmp_absolute_long(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let target_address = get_address_absolute_long(cpu, bus);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    4
}

pub fn jmp_absolute_indirect(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let pointer_address = read_offset_word(cpu, bus);
    let address = read_word(cpu, bus, pointer_address);

    cpu.registers.pc = address;

    5
}

pub fn jmp_absolute_indexed_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let base_pointer = read_offset_word(cpu, bus);
    let pointer_address = base_pointer + get_x_register_value(cpu);
    let address = read_word(cpu, bus, pointer_address);

    cpu.registers.pc = address;

    6
}

pub fn jmp_absolute_indirect_long(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let pointer_address = read_offset_word(cpu, bus);
    let address_low = read_byte(cpu, bus, pointer_address + 1);
    let address_mid = read_byte(cpu, bus, pointer_address + 2);
    let address_high = read_byte(cpu, bus, pointer_address + 3);
    let target_address =
        (address_high as u32) << 16 | (address_mid as u32) << 8 | (address_low as u32);

    cpu.registers.pc = (target_address & 0xFFFF) as u16;
    cpu.registers.pb = ((target_address >> 16) & 0xFF) as u8;

    6
}
