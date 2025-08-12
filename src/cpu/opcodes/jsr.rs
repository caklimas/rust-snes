use crate::{
    cpu::{
        Cpu,
        opcodes::{push_byte, read_offset_word},
    },
    memory::bus::Bus,
};

pub fn jsr_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address = read_offset_word(cpu, bus);
    let return_address = cpu.registers.pc + 2;

    push_byte(cpu, bus, ((return_address >> 8) & 0xFF) as u8);
    push_byte(cpu, bus, (return_address & 0xFF) as u8);

    cpu.registers.pc = address;

    6
}
