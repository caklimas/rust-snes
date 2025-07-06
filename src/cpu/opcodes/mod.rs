use crate::{
    cpu::{Cpu, processor_status::ProcessorStatus},
    memory::bus::Bus,
};

pub mod lda;

pub fn execute_opcode(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> u8 {
    match opcode {
        0xA1 => lda::lda_indirect_x(cpu, bus),
        0xA5 => lda::lda_direct(cpu, bus),
        0xA9 => lda::lda_immediate(cpu, bus),
        0xAD => lda::lda_absolute(cpu, bus),
        0xB1 => lda::lda_indirect_y(cpu, bus),
        0xB2 => lda::lda_indirect(cpu, bus),
        0xB5 => lda::lda_direct_x(cpu, bus),
        0xB9 => lda::lda_absolute_y(cpu, bus),
        0xBD => lda::lda_absolute_x(cpu, bus),
        _ => {
            println!(
                "Unimplemented opcode: 0x{:02X} at PC: 0x{:04X}",
                opcode, cpu.registers.pc
            );
            std::process::exit(1);
        }
    }
}

fn read_word(bus: &mut Bus, address: u32) -> u16 {
    let low = read_byte(bus, address);
    let high = read_byte(bus, address + 1);
    (high as u16) << 8 | (low as u16)
}

fn read_byte(bus: &mut Bus, address: u32) -> u8 {
    bus.read(address)
}

fn is_negative_u8(value: u8) -> bool {
    value & 0x80 != 0
}

fn is_negative_u16(value: u16) -> bool {
    value & 0x8000 != 0
}

fn is_8bit_mode(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::MEMORY_WIDTH)
}
