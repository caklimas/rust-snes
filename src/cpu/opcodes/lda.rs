use crate::{
    cpu::{
        Cpu,
        opcodes::{is_8bit_mode, is_negative_u8, is_negative_u16, read_byte, read_word},
        processor_status::ProcessorStatus,
    },
    memory::bus::Bus,
};

pub fn lda_immediate(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let mut pc_increment = 2;
    let address = (cpu.registers.pc + 1) as u32;
    if is_8bit_mode(cpu) {
        let value = read_byte(bus, address);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | value as u16;
        cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
        cpu.registers
            .p
            .set(ProcessorStatus::NEGATIVE, is_negative_u8(value));
    } else {
        let value = read_word(bus, address);
        cpu.registers.a = value;
        cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
        cpu.registers
            .p
            .set(ProcessorStatus::NEGATIVE, is_negative_u16(value));
        pc_increment += 1;
    }

    cpu.registers.pc += pc_increment;

    if is_8bit_mode(cpu) { 2 } else { 3 }
}

pub fn lda_absolute(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let address_low = read_byte(bus, (cpu.registers.pc + 1) as u32);
    let address_high = read_byte(bus, (cpu.registers.pc + 2) as u32);
    let target_address = ((address_high as u16) << 8 | (address_low as u16)) as u32;
    let is_8_bit_mode = is_8bit_mode(cpu);
    let mut cycles = 0;

    if is_8_bit_mode {
        let value = read_byte(bus, target_address) as u16;
        cpu.registers.a = (cpu.registers.a & 0xFF00) | value;
        cycles = 4
    } else {
        let value_low = read_byte(bus, target_address) as u16;
        let value_high = read_byte(bus, target_address + 1) as u16;
        cpu.registers.a = (value_high << 8) | value_low;

        cycles = 5;
    }

    cycles
}
