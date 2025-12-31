use crate::{
    cpu::{
        Cpu,
        opcodes::{direct_page_low_is_zero, get_x_register_value},
    },
    memory::MemoryBus,
};

/// Read a byte from program space at PC + 1
pub(crate) fn read_offset_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u8 {
    read_byte(cpu, bus, cpu.registers.pc.wrapping_add(1))
}

/// Read a word from program space at PC + 1 and PC + 2
pub(crate) fn read_offset_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset_low = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(1));
    let offset_high = read_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));

    (offset_high as u16) << 8 | (offset_low as u16)
}

/// Read a word from program space at the specified address
pub(crate) fn read_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u16 {
    let low = read_byte(cpu, bus, address);
    let high = read_byte(cpu, bus, address.wrapping_add(1));
    (high as u16) << 8 | (low as u16)
}

pub(crate) fn read_byte_direct_page<B: MemoryBus>(bus: &mut B, address: u16) -> u8 {
    bus.read(address as u32)
}

/// Read a word from direct page (bank 0)
pub(crate) fn read_word_direct_page<B: MemoryBus>(bus: &mut B, address: u16) -> u16 {
    let low = read_byte_direct_page(bus, address);
    let high = read_byte_direct_page(bus, address.wrapping_add(1));
    ((high as u16) << 8) | (low as u16)
}

/// Read a 24-bit pointer from direct page (bank 0)
pub(crate) fn read_long_pointer_direct_page<B: MemoryBus>(bus: &mut B, dp_addr: u16) -> u32 {
    // Reads 24-bit pointer from bank 0 at dp_addr..dp_addr+2
    let lo = bus.read(dp_addr as u32);
    let hi = bus.read(dp_addr.wrapping_add(1) as u32);
    let bank = bus.read(dp_addr.wrapping_add(2) as u32);

    let addr16 = u16::from_le_bytes([lo, hi]);
    ((bank as u32) << 16) | (addr16 as u32)
}

pub(crate) fn read_long_pointer_direct_page_wrapped<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
    dp_base: u16,
) -> u32 {
    // In emulation mode with page-aligned D (D.l == 0), SST expects DP wrapping for dp+1/dp+2
    if cpu.emulation_mode && direct_page_low_is_zero(cpu) {
        let page = dp_base & 0xFF00;
        let lo_addr = dp_base;
        let hi_addr = page | ((dp_base.wrapping_add(1)) as u8 as u16);
        let bank_addr = page | ((dp_base.wrapping_add(2)) as u8 as u16);

        let lo = bus.read(lo_addr as u32);
        let hi = bus.read(hi_addr as u32);
        let bank = bus.read(bank_addr as u32);

        let addr16 = u16::from_le_bytes([lo, hi]);
        ((bank as u32) << 16) | (addr16 as u32)
    } else {
        read_long_pointer_direct_page(bus, dp_base)
    }
}

pub(crate) fn get_address_absolute_x_data_physical<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16, u32) {
    let base = read_offset_word(cpu, bus);
    let x = get_x_register_value(cpu);

    let eff16 = base.wrapping_add(x);
    let base_phys = ((cpu.registers.db as u32) << 16) | (base as u32);

    let eff_phys = (base_phys.wrapping_add(x as u32)) & 0x00FF_FFFF;

    (base, eff16, eff_phys)
}

/// Read a byte from data space (uses Data Bank)
pub(crate) fn read_data_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u8 {
    let physical = ((cpu.registers.db as u32) << 16) | (address as u32);
    bus.read(physical)
}

/// Read a word from data space (uses Data Bank)
pub(crate) fn read_data_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u16 {
    let lo = read_data_byte(cpu, bus, address);
    let hi = read_data_byte(cpu, bus, address.wrapping_add(1));
    u16::from_le_bytes([lo, hi])
}

pub(crate) fn read_phys_byte<B: MemoryBus>(bus: &mut B, phys: u32) -> u8 {
    bus.read(phys)
}

pub(crate) fn read_phys_word<B: MemoryBus>(bus: &mut B, physical_address: u32) -> u16 {
    let lo = bus.read(physical_address);
    let hi = bus.read((physical_address.wrapping_add(1)) & 0x00FF_FFFF);
    u16::from_le_bytes([lo, hi])
}

/// Write a byte to direct page (bank 0)
pub(crate) fn write_byte_direct_page<B: MemoryBus>(bus: &mut B, address: u16, value: u8) {
    bus.write(address as u32, value);
}

/// Write a word to direct page (bank 0)
pub(crate) fn write_word_direct_page<B: MemoryBus>(bus: &mut B, address: u16, value: u16) {
    bus.write(address as u32, (value & 0x00FF) as u8);
    bus.write(address.wrapping_add(1) as u32, (value >> 8) as u8);
}

/// Read from program space (uses Program Bank for instruction operands)
pub(crate) fn read_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16) -> u8 {
    let physical_address = ((cpu.registers.pb as u32) << 16) | (address as u32);
    bus.read(physical_address)
}

/// Write a word to data space (uses Data Bank)
pub(crate) fn write_word<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16, value: u16) {
    write_byte(cpu, bus, address, value as u8);
    write_byte(
        cpu,
        bus,
        address.wrapping_add(1),
        ((value >> 8) & 0xFF) as u8,
    );
}

/// Write a byte to data space (uses Data Bank)
pub(crate) fn write_byte<B: MemoryBus>(cpu: &Cpu, bus: &mut B, address: u16, value: u8) {
    let physical_address = ((cpu.registers.db as u32) << 16) | (address as u32);
    bus.write(physical_address, value);
}
