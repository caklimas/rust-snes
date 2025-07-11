use crate::{
    cpu::{Cpu, processor_status::ProcessorStatus},
    memory::bus::Bus,
};

pub mod adc;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod sta;
pub mod stx;
pub mod sty;

pub fn execute_opcode(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> u8 {
    match opcode {
        0x65 => adc::adc_direct(cpu, bus),
        0x69 => adc::adc_immediate(cpu, bus),
        0x81 => sta::sta_indirect_x(cpu, bus),
        0x84 => sty::sty_direct(cpu, bus),
        0x85 => sta::sta_direct(cpu, bus),
        0x86 => stx::stx_direct(cpu, bus),
        0x8C => sty::sty_absolute(cpu, bus),
        0x8D => sta::sta_absolute(cpu, bus),
        0x8E => stx::stx_absolute(cpu, bus),
        0x91 => sta::sta_indirect_y(cpu, bus),
        0x92 => sta::sta_indirect(cpu, bus),
        0x94 => sty::sty_direct_x(cpu, bus),
        0x95 => sta::sta_direct_x(cpu, bus),
        0x96 => stx::stx_direct_y(cpu, bus),
        0x99 => sta::sta_absolute_y(cpu, bus),
        0x9D => sta::sta_absolute_x(cpu, bus),
        0xA0 => ldy::ldy_immediate(cpu, bus),
        0xA1 => lda::lda_indirect_x(cpu, bus),
        0xA2 => ldx::ldx_immediate(cpu, bus),
        0xA4 => ldy::ldy_direct(cpu, bus),
        0xA5 => lda::lda_direct(cpu, bus),
        0xA6 => ldx::ldx_direct(cpu, bus),
        0xA9 => lda::lda_immediate(cpu, bus),
        0xAC => ldy::ldy_absolute(cpu, bus),
        0xAD => lda::lda_absolute(cpu, bus),
        0xAE => ldx::ldx_absolute(cpu, bus),
        0xB1 => lda::lda_indirect_y(cpu, bus),
        0xB2 => lda::lda_indirect(cpu, bus),
        0xB4 => ldy::ldy_direct_x(cpu, bus),
        0xB5 => lda::lda_direct_x(cpu, bus),
        0xB6 => ldx::ldx_direct_y(cpu, bus),
        0xB9 => lda::lda_absolute_y(cpu, bus),
        0xBC => ldy::ldy_absolute_x(cpu, bus),
        0xBD => lda::lda_absolute_x(cpu, bus),
        0xBE => ldx::ldx_absolute_y(cpu, bus),
        _ => {
            println!(
                "Unimplemented opcode: 0x{:02X} at PC: 0x{:04X}",
                opcode, cpu.registers.pc
            );
            std::process::exit(1);
        }
    }
}

fn read_offset_byte(cpu: &Cpu, bus: &mut Bus) -> u16 {
    read_byte(bus, (cpu.registers.pc + 1).into()).into()
}

fn read_offset_word(cpu: &Cpu, bus: &mut Bus) -> u16 {
    let offset_low = read_byte(bus, (cpu.registers.pc + 1).into());
    let offset_high = read_byte(bus, (cpu.registers.pc + 2).into());

    (offset_high as u16) << 8 | (offset_low as u16)
}

fn read_word(bus: &mut Bus, address: u32) -> u16 {
    let low = read_byte(bus, address);
    let high = read_byte(bus, address + 1);
    (high as u16) << 8 | (low as u16)
}

fn read_byte(bus: &mut Bus, address: u32) -> u8 {
    bus.read(address)
}

fn write_word(bus: &mut Bus, address: u32, value: u16) {
    write_byte(bus, address, (value as u8) & 0xFF);
    write_byte(bus, address + 1, ((value >> 8) & 0xFF) as u8);
}

fn write_byte(bus: &mut Bus, address: u32, value: u8) {
    bus.write(address, value);
}

fn is_8bit_mode_m(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::MEMORY_WIDTH)
}

fn is_8bit_mode_x(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::INDEX_WIDTH)
}

fn increment_program_counter(cpu: &mut Cpu, value: u16) {
    cpu.registers.pc += value;
}

fn set_nz_flags_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, is_negative_u8(value));
}

fn set_nz_flags_u16(cpu: &mut Cpu, value: u16) {
    cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, is_negative_u16(value));
}

fn is_negative_u8(value: u8) -> bool {
    value & 0x80 != 0
}

fn is_negative_u16(value: u16) -> bool {
    value & 0x8000 != 0
}

fn page_crossed(base_address: u16, target_address: u16) -> bool {
    (base_address & 0xFF00) != (target_address & 0xFF00)
}

fn get_carry_in(cpu: &Cpu) -> u16 {
    if cpu.registers.p.contains(ProcessorStatus::CARRY) {
        1
    } else {
        0
    }
}
