use crate::{
    cpu::{Cpu, processor_status::ProcessorStatus},
    memory::{
        addresses::{DIRECT_PAGE_START, STACK_START},
        bus::Bus,
    },
};

pub mod adc;
pub mod bra;
pub mod cmp;
pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod sbc;
pub mod sta;
pub mod stack;
pub mod stx;
pub mod sty;
pub mod transfer;

pub fn execute_opcode(cpu: &mut Cpu, bus: &mut Bus, opcode: u8) -> u8 {
    match opcode {
        0x08 => stack::php(cpu, bus),
        0x0B => stack::phd(cpu, bus),
        0x10 => bra::bpl(cpu, bus),
        0x1B => transfer::tcs(cpu, bus),
        0x20 => jsr::jsr_absolute(cpu, bus),
        0x22 => jsr::jsr_absolute_long(cpu, bus),
        0x28 => stack::plp(cpu, bus),
        0x2B => stack::pld(cpu, bus),
        0x30 => bra::bmi(cpu, bus),
        0x3B => transfer::tsc(cpu, bus),
        0x48 => stack::pha(cpu, bus),
        0x4B => stack::phk(cpu, bus),
        0x4C => jmp::jmp_absolute(cpu, bus),
        0x50 => bra::bvc(cpu, bus),
        0x5A => stack::phy(cpu, bus),
        0x5B => transfer::tcd(cpu, bus),
        0x5C => jmp::jmp_absolute_long(cpu, bus),
        0x61 => adc::adc_indirect_x(cpu, bus),
        0x62 => stack::per(cpu, bus),
        0x65 => adc::adc_direct(cpu, bus),
        0x68 => stack::pla(cpu, bus),
        0x69 => adc::adc_immediate(cpu, bus),
        0x6C => jmp::jmp_absolute_indirect(cpu, bus),
        0x6D => adc::adc_absolute(cpu, bus),
        0x70 => bra::bvs(cpu, bus),
        0x71 => adc::adc_indirect_y(cpu, bus),
        0x72 => adc::adc_indirect(cpu, bus),
        0x75 => adc::adc_direct_x(cpu, bus),
        0x79 => adc::adc_absolute_y(cpu, bus),
        0x7A => stack::ply(cpu, bus),
        0x7B => transfer::tdc(cpu, bus),
        0x7C => jmp::jmp_absolute_indexed_direct(cpu, bus),
        0x7D => adc::adc_absolute_x(cpu, bus),
        0x80 => bra::bra_relative(cpu, bus),
        0x81 => sta::sta_indirect_x(cpu, bus),
        0x82 => bra::bra_relative_long(cpu, bus),
        0x84 => sty::sty_direct(cpu, bus),
        0x85 => sta::sta_direct(cpu, bus),
        0x86 => stx::stx_direct(cpu, bus),
        0x8A => transfer::txa(cpu, bus),
        0x8B => stack::phb(cpu, bus),
        0x8C => sty::sty_absolute(cpu, bus),
        0x8D => sta::sta_absolute(cpu, bus),
        0x8E => stx::stx_absolute(cpu, bus),
        0x90 => bra::bcc(cpu, bus),
        0x91 => sta::sta_indirect_y(cpu, bus),
        0x92 => sta::sta_indirect(cpu, bus),
        0x94 => sty::sty_direct_x(cpu, bus),
        0x95 => sta::sta_direct_x(cpu, bus),
        0x96 => stx::stx_direct_y(cpu, bus),
        0x98 => transfer::tya(cpu, bus),
        0x99 => sta::sta_absolute_y(cpu, bus),
        0x9A => transfer::txs(cpu, bus),
        0x9B => transfer::txy(cpu, bus),
        0x9D => sta::sta_absolute_x(cpu, bus),
        0xA0 => ldy::ldy_immediate(cpu, bus),
        0xA1 => lda::lda_indirect_x(cpu, bus),
        0xA2 => ldx::ldx_immediate(cpu, bus),
        0xA4 => ldy::ldy_direct(cpu, bus),
        0xA5 => lda::lda_direct(cpu, bus),
        0xA6 => ldx::ldx_direct(cpu, bus),
        0xA8 => transfer::tay(cpu, bus),
        0xA9 => lda::lda_immediate(cpu, bus),
        0xAA => transfer::tax(cpu, bus),
        0xAB => stack::plb(cpu, bus),
        0xAC => ldy::ldy_absolute(cpu, bus),
        0xAD => lda::lda_absolute(cpu, bus),
        0xAE => ldx::ldx_absolute(cpu, bus),
        0xB0 => bra::bcs(cpu, bus),
        0xB1 => lda::lda_indirect_y(cpu, bus),
        0xB2 => lda::lda_indirect(cpu, bus),
        0xB4 => ldy::ldy_direct_x(cpu, bus),
        0xB5 => lda::lda_direct_x(cpu, bus),
        0xB6 => ldx::ldx_direct_y(cpu, bus),
        0xB9 => lda::lda_absolute_y(cpu, bus),
        0xBA => transfer::tsx(cpu, bus),
        0xBB => transfer::tyx(cpu, bus),
        0xBC => ldy::ldy_absolute_x(cpu, bus),
        0xBD => lda::lda_absolute_x(cpu, bus),
        0xBE => ldx::ldx_absolute_y(cpu, bus),
        0xC1 => cmp::cmp_indirect_x(cpu, bus),
        0xC5 => cmp::cmp_direct(cpu, bus),
        0xC9 => cmp::cmp_immediate(cpu, bus),
        0xCD => cmp::cmp_absolute(cpu, bus),
        0xD0 => bra::bne(cpu, bus),
        0xD1 => cmp::cmp_indirect_y(cpu, bus),
        0xD2 => cmp::cmp_indirect(cpu, bus),
        0xD4 => stack::pei(cpu, bus),
        0xD5 => cmp::cmp_direct_x(cpu, bus),
        0xD9 => cmp::cmp_absolute_y(cpu, bus),
        0xDA => stack::phx(cpu, bus),
        0xDC => jmp::jmp_absolute_indirect_long(cpu, bus),
        0xDD => cmp::cmp_absolute_x(cpu, bus),
        0xE9 => sbc::sbc_immediate(cpu, bus),
        0xE5 => sbc::sbc_direct(cpu, bus),
        0xED => sbc::sbc_absolute(cpu, bus),
        0xE1 => sbc::sbc_indirect_x(cpu, bus),
        0xF0 => bra::beq(cpu, bus),
        0xF1 => sbc::sbc_indirect_y(cpu, bus),
        0xF2 => sbc::sbc_indirect(cpu, bus),
        0xF4 => stack::pea(cpu, bus),
        0xF5 => sbc::sbc_direct_x(cpu, bus),
        0xF9 => sbc::sbc_absolute_y(cpu, bus),
        0xFA => stack::plx(cpu, bus),
        0xFD => sbc::sbc_absolute_x(cpu, bus),
        _ => {
            println!(
                "Unimplemented opcode: 0x{:02X} at PC: 0x{:04X}",
                opcode, cpu.registers.pc
            );
            std::process::exit(1);
        }
    }
}

fn get_address_absolute_x(cpu: &Cpu, bus: &mut Bus) -> (u16, u16) {
    let base_address = read_offset_word(cpu, bus);
    (base_address, base_address + get_x_register_value(cpu))
}

fn get_address_absolute_y(cpu: &Cpu, bus: &mut Bus) -> (u16, u16) {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.y;

    (base_address, address)
}

fn get_address_indirect_x(cpu: &Cpu, bus: &mut Bus) -> u16 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset + get_x_register_value(cpu);
    read_word(cpu, bus, pointer_address)
}

fn get_address_indirect_y(cpu: &Cpu, bus: &mut Bus) -> (u16, u16) {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    let base_address = read_word(cpu, bus, pointer_address);
    let address = base_address + cpu.registers.y;

    (base_address, address)
}

fn get_address_indirect(cpu: &Cpu, bus: &mut Bus) -> u16 {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d + offset;
    read_word(cpu, bus, pointer_address)
}

fn get_address_absolute_long(cpu: &Cpu, bus: &mut Bus) -> u32 {
    let address_low = read_byte(cpu, bus, cpu.registers.pc + 1);
    let address_mid = read_byte(cpu, bus, cpu.registers.pc + 2);
    let address_high = read_byte(cpu, bus, cpu.registers.pc + 3);

    (address_high as u32) << 16 | (address_mid as u32) << 8 | (address_low as u32)
}

fn get_x_register_value(cpu: &Cpu) -> u16 {
    if is_8bit_mode_x(cpu) {
        cpu.registers.x & 0xFF
    } else {
        cpu.registers.x
    }
}

pub(crate) fn push_byte(cpu: &mut Cpu, bus: &mut Bus, value: u8) {
    let stack_address = get_stack_address(cpu);

    write_byte(cpu, bus, stack_address, value);
    cpu.registers.s = cpu.registers.s.wrapping_sub(1);
}

pub(crate) fn pull_byte(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    cpu.registers.s = cpu.registers.s.wrapping_add(1);
    let stack_address = get_stack_address(cpu);

    read_byte(cpu, bus, stack_address)
}

pub(crate) fn read_offset_byte(cpu: &Cpu, bus: &mut Bus) -> u16 {
    read_byte(cpu, bus, cpu.registers.pc + 1 ).into()
}

pub(crate) fn read_offset_word(cpu: &Cpu, bus: &mut Bus) -> u16 {
    let offset_low = read_byte(cpu, bus, cpu.registers.pc + 1);
    let offset_high = read_byte(cpu, bus, cpu.registers.pc + 2);

    (offset_high as u16) << 8 | (offset_low as u16)
}

pub(crate) fn read_word(cpu: &Cpu, bus: &mut Bus, address: u16) -> u16 {
    let low = read_byte(cpu, bus, address);
    let high = read_byte(cpu, bus, address + 1);
    (high as u16) << 8 | (low as u16)
}

fn read_byte(cpu: &Cpu, bus: &mut Bus, address: u16) -> u8 {
    let physical_address = get_physical_address(cpu, address);
    bus.read(physical_address)
}

fn write_word(cpu: &Cpu, bus: &mut Bus, address: u16, value: u16) {
    write_byte(cpu, bus, address, value as u8);
    write_byte(cpu, bus, address + 1, ((value >> 8) & 0xFF) as u8);
}

fn write_byte(cpu: &Cpu, bus: &mut Bus, address: u16, value: u8) {
    let physical_address = get_physical_address(cpu, address);
    bus.write(physical_address, value);
}

pub(crate) fn is_8bit_mode_m(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::MEMORY_WIDTH)
}

pub(crate) fn is_8bit_mode_x(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::INDEX_WIDTH)
}

pub(crate) fn increment_program_counter(cpu: &mut Cpu, value: u16) {
    cpu.registers.pc += value;
}

pub(crate) fn set_nz_flags_u8(cpu: &mut Cpu, value: u8) {
    cpu.registers.p.set(ProcessorStatus::ZERO, value == 0);
    cpu.registers
        .p
        .set(ProcessorStatus::NEGATIVE, is_negative_u8(value));
}

pub(crate) fn set_nz_flags_u16(cpu: &mut Cpu, value: u16) {
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

fn get_physical_address(cpu: &Cpu, address: u16) -> u32 {
    ((cpu.registers.db as u32) << 16) | (address as u32)
}

fn get_stack_address(cpu: &Cpu) -> u16 {
    if cpu.emulation_mode {
        STACK_START as u16 | (cpu.registers.s & 0xFF)
    } else {
        DIRECT_PAGE_START as u16 | cpu.registers.s
    }
}
