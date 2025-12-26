use crate::{
    cpu::{Cpu, processor_status::ProcessorStatus},
    memory::{MemoryBus, addresses::STACK_START},
};

pub mod adc;
pub mod and;
pub mod bit;
pub mod block_move;
pub mod bra;
pub mod cmp;
pub mod cpx;
pub mod cpy;
pub mod dec;
pub mod eor;
pub mod flags;
pub mod inc;
pub mod jmp;
pub mod jsr;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod misc;
pub mod ora;
pub mod ret;
pub mod sbc;
pub mod shift;
pub mod sta;
pub mod stack;
pub mod stx;
pub mod sty;
pub mod stz;
pub mod transfer;

#[derive(Copy, Clone)]
pub enum StackMode {
    // Normal 6502-style emulation stack behavior: $01xx + 8-bit SP
    EmuPage1,
    // SingleStepTests behavior for certain "new" 65816 ops: linear 16-bit SP
    Linear16,
}

pub fn execute_opcode<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, opcode: u8) -> u8 {
    let mode = if cpu.emulation_mode {
        StackMode::EmuPage1
    } else {
        StackMode::Linear16
    };

    match opcode {
        0x00 => misc::brk(cpu, bus, mode),
        0x01 => ora::ora_indirect_x(cpu, bus),
        0x02 => misc::cop(cpu, bus, mode),
        0x03 => ora::ora_stack_relative(cpu, bus),
        0x04 => bit::tsb_direct(cpu, bus),
        0x05 => ora::ora_direct(cpu, bus),
        0x06 => shift::asl_direct(cpu, bus),
        0x07 => ora::ora_indirect_long(cpu, bus),
        0x08 => stack::php(cpu, bus, mode),
        0x09 => ora::ora_immediate(cpu, bus),
        0x0A => shift::asl_accumulator(cpu, bus),
        0x0B => stack::phd(cpu, bus),
        0x0C => bit::tsb_absolute(cpu, bus),
        0x0D => ora::ora_absolute(cpu, bus),
        0x0E => shift::asl_absolute(cpu, bus),
        0x0F => ora::ora_absolute_long(cpu, bus),

        0x10 => bra::bpl(cpu, bus),
        0x11 => ora::ora_indirect_y(cpu, bus),
        0x12 => ora::ora_indirect(cpu, bus),
        0x13 => ora::ora_stack_relative_indirect_y(cpu, bus),
        0x14 => bit::trb_direct(cpu, bus),
        0x15 => ora::ora_direct_x(cpu, bus),
        0x16 => shift::asl_direct_x(cpu, bus),
        0x18 => flags::clc(cpu, bus),
        0x19 => ora::ora_absolute_y(cpu, bus),
        0x1A => inc::ina(cpu, bus),
        0x1B => transfer::tcs(cpu, bus),
        0x1C => bit::trb_absolute(cpu, bus),
        0x1D => ora::ora_absolute_x(cpu, bus),
        0x1E => shift::asl_absolute_x(cpu, bus),

        0x20 => jsr::jsr_absolute(cpu, bus, mode),
        0x21 => and::and_indirect_x(cpu, bus),
        0x22 => jsr::jsr_absolute_long(cpu, bus),
        0x24 => bit::bit_direct(cpu, bus),
        0x25 => and::and_direct(cpu, bus),
        0x26 => shift::rol_direct(cpu, bus),
        0x28 => stack::plp(cpu, bus, mode),
        0x29 => and::and_immediate(cpu, bus),
        0x2A => shift::rol_accumulator(cpu, bus),
        0x2B => stack::pld(cpu, bus),
        0x2C => bit::bit_absolute(cpu, bus),
        0x2D => and::and_absolute(cpu, bus),
        0x2E => shift::rol_absolute(cpu, bus),

        0x30 => bra::bmi(cpu, bus),
        0x31 => and::and_indirect_y(cpu, bus),
        0x32 => and::and_indirect(cpu, bus),
        0x34 => bit::bit_direct_x(cpu, bus),
        0x35 => and::and_direct_x(cpu, bus),
        0x36 => shift::rol_direct_x(cpu, bus),
        0x38 => flags::sec(cpu, bus),
        0x39 => and::and_absolute_y(cpu, bus),
        0x3A => dec::dea(cpu, bus),
        0x3B => transfer::tsc(cpu, bus),
        0x3C => bit::bit_absolute_x(cpu, bus),
        0x3D => and::and_absolute_x(cpu, bus),
        0x3E => shift::rol_absolute_x(cpu, bus),

        0x40 => ret::rti(cpu, bus, mode),
        0x41 => eor::eor_indirect_x(cpu, bus),
        0x42 => misc::wdm(cpu, bus),
        0x44 => block_move::mvp(cpu, bus),
        0x45 => eor::eor_direct(cpu, bus),
        0x46 => shift::lsr_direct(cpu, bus),
        0x48 => stack::pha(cpu, bus, mode),
        0x49 => eor::eor_immediate(cpu, bus),
        0x4A => shift::lsr_accumulator(cpu, bus),
        0x4B => stack::phk(cpu, bus, mode),
        0x4C => jmp::jmp_absolute(cpu, bus),
        0x4D => eor::eor_absolute(cpu, bus),
        0x4E => shift::lsr_absolute(cpu, bus),

        0x50 => bra::bvc(cpu, bus),
        0x51 => eor::eor_indirect_y(cpu, bus),
        0x52 => eor::eor_indirect(cpu, bus),
        0x54 => block_move::mvn(cpu, bus),
        0x55 => eor::eor_direct_x(cpu, bus),
        0x56 => shift::lsr_direct_x(cpu, bus),
        0x58 => flags::cli(cpu, bus),
        0x59 => eor::eor_absolute_y(cpu, bus),
        0x5A => stack::phy(cpu, bus, mode),
        0x5B => transfer::tcd(cpu, bus),
        0x5C => jmp::jmp_absolute_long(cpu, bus),
        0x5D => eor::eor_absolute_x(cpu, bus),
        0x5E => shift::lsr_absolute_x(cpu, bus),

        0x60 => ret::rts(cpu, bus, mode),
        0x61 => adc::adc_indirect_x(cpu, bus),
        0x62 => stack::per(cpu, bus),
        0x64 => stz::stz_direct(cpu, bus),
        0x65 => adc::adc_direct(cpu, bus),
        0x66 => shift::ror_direct(cpu, bus),
        0x68 => stack::pla(cpu, bus, mode),
        0x69 => adc::adc_immediate(cpu, bus),
        0x6A => shift::ror_accumulator(cpu, bus),
        0x6B => ret::rtl(cpu, bus),
        0x6C => jmp::jmp_absolute_indirect(cpu, bus),
        0x6D => adc::adc_absolute(cpu, bus),
        0x6E => shift::ror_absolute(cpu, bus),

        0x70 => bra::bvs(cpu, bus),
        0x71 => adc::adc_indirect_y(cpu, bus),
        0x72 => adc::adc_indirect(cpu, bus),
        0x74 => stz::stz_direct_x(cpu, bus),
        0x75 => adc::adc_direct_x(cpu, bus),
        0x76 => shift::ror_direct_x(cpu, bus),
        0x78 => flags::sei(cpu, bus),
        0x79 => adc::adc_absolute_y(cpu, bus),
        0x7A => stack::ply(cpu, bus, mode),
        0x7B => transfer::tdc(cpu, bus),
        0x7C => jmp::jmp_absolute_indexed_direct(cpu, bus),
        0x7D => adc::adc_absolute_x(cpu, bus),
        0x7E => shift::ror_absolute_x(cpu, bus),

        0x80 => bra::bra_relative(cpu, bus),
        0x81 => sta::sta_indirect_x(cpu, bus),
        0x82 => bra::bra_relative_long(cpu, bus),
        0x84 => sty::sty_direct(cpu, bus),
        0x85 => sta::sta_direct(cpu, bus),
        0x86 => stx::stx_direct(cpu, bus),
        0x88 => dec::dey(cpu, bus),
        0x89 => bit::bit_immediate(cpu, bus),
        0x8A => transfer::txa(cpu, bus),
        0x8B => stack::phb(cpu, bus, mode),
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
        0x9C => stz::stz_absolute(cpu, bus),
        0x9D => sta::sta_absolute_x(cpu, bus),
        0x9E => stz::stz_absolute_x(cpu, bus),

        0xA0 => ldy::ldy_immediate(cpu, bus),
        0xA1 => lda::lda_indirect_x(cpu, bus),
        0xA2 => ldx::ldx_immediate(cpu, bus),
        0xA4 => ldy::ldy_direct(cpu, bus),
        0xA5 => lda::lda_direct(cpu, bus),
        0xA6 => ldx::ldx_direct(cpu, bus),
        0xA8 => transfer::tay(cpu, bus),
        0xA9 => lda::lda_immediate(cpu, bus),
        0xAA => transfer::tax(cpu, bus),
        0xAB => stack::plb(cpu, bus, mode),
        0xAC => ldy::ldy_absolute(cpu, bus),
        0xAD => lda::lda_absolute(cpu, bus),
        0xAE => ldx::ldx_absolute(cpu, bus),

        0xB0 => bra::bcs(cpu, bus),
        0xB1 => lda::lda_indirect_y(cpu, bus),
        0xB2 => lda::lda_indirect(cpu, bus),
        0xB4 => ldy::ldy_direct_x(cpu, bus),
        0xB5 => lda::lda_direct_x(cpu, bus),
        0xB6 => ldx::ldx_direct_y(cpu, bus),
        0xB8 => flags::clv(cpu, bus),
        0xB9 => lda::lda_absolute_y(cpu, bus),
        0xBA => transfer::tsx(cpu, bus),
        0xBB => transfer::tyx(cpu, bus),
        0xBC => ldy::ldy_absolute_x(cpu, bus),
        0xBD => lda::lda_absolute_x(cpu, bus),
        0xBE => ldx::ldx_absolute_y(cpu, bus),

        0xC0 => cpy::cpy_immediate(cpu, bus),
        0xC1 => cmp::cmp_indirect_x(cpu, bus),
        0xC2 => flags::rep(cpu, bus),
        0xC4 => cpy::cpy_direct(cpu, bus),
        0xC5 => cmp::cmp_direct(cpu, bus),
        0xC6 => dec::dec_direct(cpu, bus),
        0xC8 => inc::iny(cpu, bus),
        0xC9 => cmp::cmp_immediate(cpu, bus),
        0xCA => dec::dex(cpu, bus),
        0xCB => misc::wai(cpu, bus),
        0xCC => cpy::cpy_absolute(cpu, bus),
        0xCD => cmp::cmp_absolute(cpu, bus),
        0xCE => dec::dec_absolute(cpu, bus),

        0xD0 => bra::bne(cpu, bus),
        0xD1 => cmp::cmp_indirect_y(cpu, bus),
        0xD2 => cmp::cmp_indirect(cpu, bus),
        0xD4 => stack::pei(cpu, bus),
        0xD5 => cmp::cmp_direct_x(cpu, bus),
        0xD6 => dec::dec_direct_x(cpu, bus),
        0xD8 => flags::cld(cpu, bus),
        0xD9 => cmp::cmp_absolute_y(cpu, bus),
        0xDA => stack::phx(cpu, bus, mode),
        0xDB => misc::stp(cpu, bus),
        0xDC => jmp::jmp_absolute_indirect_long(cpu, bus),
        0xDD => cmp::cmp_absolute_x(cpu, bus),
        0xDE => dec::dec_absolute_x(cpu, bus),

        0xE0 => cpx::cpx_immediate(cpu, bus),
        0xE1 => sbc::sbc_indirect_x(cpu, bus),
        0xE2 => flags::sep(cpu, bus),
        0xE4 => cpx::cpx_direct(cpu, bus),
        0xE5 => sbc::sbc_direct(cpu, bus),
        0xE6 => inc::inc_direct(cpu, bus),
        0xE8 => inc::inx(cpu, bus),
        0xE9 => sbc::sbc_immediate(cpu, bus),
        0xEA => misc::nop(cpu, bus),
        0xEB => misc::xba(cpu, bus),
        0xEC => cpx::cpx_absolute(cpu, bus),
        0xED => sbc::sbc_absolute(cpu, bus),
        0xEE => inc::inc_absolute(cpu, bus),

        0xF0 => bra::beq(cpu, bus),
        0xF1 => sbc::sbc_indirect_y(cpu, bus),
        0xF2 => sbc::sbc_indirect(cpu, bus),
        0xF4 => stack::pea(cpu, bus),
        0xF5 => sbc::sbc_direct_x(cpu, bus),
        0xF6 => inc::inc_direct_x(cpu, bus),
        0xF8 => flags::sed(cpu, bus),
        0xF9 => sbc::sbc_absolute_y(cpu, bus),
        0xFA => stack::plx(cpu, bus, mode),
        0xFB => misc::xce(cpu, bus),
        0xFD => sbc::sbc_absolute_x(cpu, bus),
        0xFE => inc::inc_absolute_x(cpu, bus),

        _ => {
            println!(
                "Unimplemented opcode: 0x{:02X} at PC: 0x{:04X}",
                opcode, cpu.registers.pc
            );
            std::process::exit(1);
        }
    }
}

pub(crate) fn push_byte<B: MemoryBus>(
    cpu: &mut Cpu,
    bus: &mut B,
    value: u8,
    stack_mode: StackMode,
) {
    let stack_address = get_stack_address(cpu, stack_mode);
    // Stack is always in bank 0
    bus.write(stack_address as u32, value);
    decrement_stack_pointer(cpu, stack_mode);
}

pub(crate) fn pull_byte<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B, stack_mode: StackMode) -> u8 {
    increment_stack_pointer(cpu, stack_mode);
    let stack_address = get_stack_address(cpu, stack_mode);
    // Stack is always in bank 0
    bus.read(stack_address as u32)
}

pub(crate) fn read_offset_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u8 {
    read_byte(cpu, bus, cpu.registers.pc.wrapping_add(1))
}

pub(crate) fn read_offset_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset_low = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(1));
    let offset_high = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));

    (offset_high as u16) << 8 | (offset_low as u16)
}

pub(crate) fn read_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u16 {
    let low = read_byte(cpu, bus, address);
    let high = read_byte(cpu, bus, address.wrapping_add(1));
    (high as u16) << 8 | (low as u16)
}

pub(crate) fn read_word_direct_page<B: MemoryBus>(bus: &mut B, address: u16) -> u16 {
    let low = bus.read(address as u32);
    let high = bus.read(address.wrapping_add(1) as u32);
    ((high as u16) << 8) | (low as u16)
}

pub(crate) fn read_long_pointer_direct_page<B: MemoryBus>(bus: &mut B, dp_addr: u16) -> u32 {
    // Reads 24-bit pointer from bank 0 at dp_addr..dp_addr+2
    let lo = bus.read(dp_addr as u32);
    let hi = bus.read(dp_addr.wrapping_add(1) as u32);
    let bank = bus.read(dp_addr.wrapping_add(2) as u32);

    let addr16 = u16::from_le_bytes([lo, hi]);
    ((bank as u32) << 16) | (addr16 as u32)
}

pub(crate) fn read_data_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u8 {
    let physical = ((cpu.registers.db as u32) << 16) | (address as u32);
    bus.read(physical)
}

pub(crate) fn read_data_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u16 {
    let lo = read_data_byte(cpu, bus, address);
    let hi = read_data_byte(cpu, bus, address.wrapping_add(1));
    u16::from_le_bytes([lo, hi])
}

pub(crate) fn write_byte_direct_page<B: MemoryBus>(bus: &mut B, address: u16, value: u8) {
    bus.write(address as u32, value);
}

pub(crate) fn write_word_direct_page<B: MemoryBus>(bus: &mut B, address: u16, value: u16) {
    bus.write(address as u32, (value & 0x00FF) as u8);
    bus.write(address.wrapping_add(1) as u32, (value >> 8) as u8);
}

pub(crate) fn calculate_direct_page_x_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> (u16, u16) {
    let offset: u8 = read_offset_byte(cpu, bus);
    let x: u8 = cpu.registers.x as u8;

    let base_address = cpu.registers.d.wrapping_add(offset as u16);

    let dp_index = offset.wrapping_add(x);
    let target_address = cpu.registers.d.wrapping_add(dp_index as u16);

    (base_address, target_address)
}

pub(crate) fn calculate_direct_page_y_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset: u8 = read_offset_byte(cpu, bus);
    let y: u8 = cpu.registers.y as u8; // always low byte for direct page indexed
    let dp_index = offset.wrapping_add(y);
    cpu.registers.d.wrapping_add(dp_index as u16)
}

pub(crate) fn calculate_direct_page_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset = read_offset_byte(cpu, bus);
    cpu.registers.d.wrapping_add(offset as u16)
}

pub(crate) fn calculate_indirect_page_x_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16, u16) {
    let offset: u8 = read_offset_byte(cpu, bus);
    let _x_index: u8 = cpu.registers.x as u8;

    // X is 8-bit if: emulation mode OR X-flag set (X=1)
    let x_index_u16: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.x as u8) as u16
    } else {
        cpu.registers.x
    };

    let direct_page: u16 = cpu.registers.d;
    let direct_page_low_byte: u8 = (direct_page & 0x00FF) as u8;
    let base_pointer_address = direct_page.wrapping_add(offset as u16);
    let pointer_address: u16 = if cpu.emulation_mode && direct_page_low_byte == 0 {
        // Emulation-mode, page-aligned direct page:
        // wrap offset+X within the 256-byte direct page window
        let wrapped_index: u8 = offset.wrapping_add(x_index_u16 as u8);
        (direct_page & 0xFF00) | (wrapped_index as u16)
    } else {
        // Native-mode or non-aligned direct page:
        // full 16-bit addition with carry
        direct_page
            .wrapping_add(offset as u16)
            .wrapping_add(x_index_u16)
    };

    let target_address: u16 = read_word_direct_page(bus, pointer_address);

    (base_pointer_address, pointer_address, target_address)
}

pub(crate) fn calculate_indirect_page_y_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16) {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d.wrapping_add(offset as u16);
    let base_address = read_word_direct_page(bus, pointer_address);
    let y = if is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let address = base_address.wrapping_add(y);

    (base_address, address)
}

pub(crate) fn calculate_indirect_page_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset: u8 = read_offset_byte(cpu, bus);
    let pointer_address: u16 = cpu.registers.d.wrapping_add(offset as u16);
    read_word_direct_page(bus, pointer_address)
}

pub(crate) fn calculate_stack_relative_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset: u8 = read_offset_byte(cpu, bus);
    cpu.registers.s.wrapping_add(offset as u16) // bank 0
}

pub(crate) fn calculate_stack_relative_indirect_y_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16) {
    let offset: u8 = read_offset_byte(cpu, bus);
    let pointer_address: u16 = cpu.registers.s.wrapping_add(offset as u16);

    // Read the 16-bit pointer from stack (always in bank 0)
    let base_address = read_word_direct_page(bus, pointer_address);

    // Add Y register to the pointer
    let y = if is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let target_address = base_address.wrapping_add(y);

    (base_address, target_address)
}

pub(crate) fn calculate_absolute_long_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u32 {
    // Reads ll, hh, bb from instruction stream using PBR
    let pc = cpu.registers.pc;

    let addr_low = read_byte(cpu, bus, pc.wrapping_add(1));
    let addr_mid = read_byte(cpu, bus, pc.wrapping_add(2));
    let addr_bank = read_byte(cpu, bus, pc.wrapping_add(3));

    ((addr_bank as u32) << 16) | ((addr_mid as u32) << 8) | (addr_low as u32)
}

fn get_address_absolute_x<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> (u16, u16) {
    let base_address = read_offset_word(cpu, bus);
    (base_address, base_address + get_x_register_value(cpu))
}

fn get_address_absolute_y<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> (u16, u16) {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + cpu.registers.y;

    (base_address, address)
}

fn get_address_absolute_long<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u32 {
    let address_low = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(1));
    let address_mid = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));
    let address_high = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(3));

    (address_high as u32) << 16 | (address_mid as u32) << 8 | (address_low as u32)
}

fn get_x_register_value(cpu: &Cpu) -> u16 {
    if is_8bit_mode_x(cpu) {
        cpu.registers.x & 0xFF
    } else {
        cpu.registers.x
    }
}

// Read from program space (uses Program Bank for instruction operands)
fn read_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u8 {
    let physical_address = ((cpu.registers.pb as u32) << 16) | (address as u32);
    bus.read(physical_address)
}

fn write_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16, value: u16) {
    write_byte(cpu, bus, address, value as u8);
    write_byte(
        cpu,
        bus,
        address.wrapping_add(1),
        ((value >> 8) & 0xFF) as u8,
    );
}

fn write_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16, value: u8) {
    let physical_address = ((cpu.registers.db as u32) << 16) | (address as u32);
    bus.write(physical_address, value);
}

pub(crate) fn is_8bit_mode_m(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::MEMORY_WIDTH)
}

pub(crate) fn is_8bit_mode_x(cpu: &Cpu) -> bool {
    cpu.registers.p.contains(ProcessorStatus::INDEX_WIDTH)
}

pub(crate) fn direct_page_low_is_zero(cpu: &Cpu) -> bool {
    cpu.registers.d & 0x00FF == 0
}

pub(crate) fn increment_program_counter(cpu: &mut Cpu, value: u16) {
    cpu.registers.pc = cpu.registers.pc.wrapping_add(value);
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

// fn get_physical_address(cpu: &Cpu, address: u16) -> u32 {
//     ((cpu.registers.db as u32) << 16) | (address as u32)
// }

fn get_stack_address(cpu: &Cpu, stack_mode: StackMode) -> u16 {
    if !cpu.emulation_mode {
        // Native mode: always linear 16-bit stack
        return cpu.registers.s;
    }

    // Emulation mode: choose behavior by StackMode
    match stack_mode {
        StackMode::EmuPage1 => (STACK_START as u16) | (cpu.registers.s & 0x00FF),
        StackMode::Linear16 => cpu.registers.s,
    }
}

fn decrement_stack_pointer(cpu: &mut Cpu, stack_mode: StackMode) {
    if !cpu.emulation_mode {
        cpu.registers.s = cpu.registers.s.wrapping_sub(1);
        return;
    }

    match stack_mode {
        StackMode::EmuPage1 => {
            let low = (cpu.registers.s as u8).wrapping_sub(1);
            cpu.registers.s = (STACK_START as u16) | low as u16;
        }
        StackMode::Linear16 => {
            cpu.registers.s = cpu.registers.s.wrapping_sub(1);
        }
    }
}

fn increment_stack_pointer(cpu: &mut Cpu, stack_mode: StackMode) {
    if !cpu.emulation_mode {
        cpu.registers.s = cpu.registers.s.wrapping_add(1);
        return;
    }

    match stack_mode {
        StackMode::EmuPage1 => {
            let low = (cpu.registers.s as u8).wrapping_add(1);
            cpu.registers.s = 0x0100 | low as u16;
        }
        StackMode::Linear16 => {
            cpu.registers.s = cpu.registers.s.wrapping_add(1);
        }
    }
}

/**
 * SST compatibility: For certain "new" 65816 ops in E=1, SST treats stack accesses as linear
 * 16-bit during execution ($0100 -> $00FF), then forces final S back to $01xx afterward.
 * Normalize S here to match SST; not representative of real hardware stack addressing.
 * https://github.com/SingleStepTests/ProcessorTests/issues/44?utm_source=chatgpt.com
 */
fn normalize_stack_pointer(cpu: &mut Cpu) {
    if cpu.emulation_mode {
        cpu.registers.s = (STACK_START as u16) | (cpu.registers.s & 0x00FF);
    }
}
