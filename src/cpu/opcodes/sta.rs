use crate::{
    cpu::{
        Cpu,
        opcodes::{is_8bit_mode, read_byte},
    },
    memory::bus::Bus,
};

pub fn sta_direct(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut cycles = 0;
    let offset = read_byte(bus, (cpu.registers.pc + 1) as u32);
    let target_address = cpu.registers.d + offset.into();

    if is_8bit_mode(cpu) {
        cycles = 3;
    } else {
        cycles - 4;
    }

    cycles;
}
