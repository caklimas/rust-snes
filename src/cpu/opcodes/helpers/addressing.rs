use crate::{
    cpu::Cpu,
    memory::{MemoryBus, addresses::STACK_START},
};

use super::{
    flags::is_8bit_mode_x,
    memory::{read_offset_byte, read_offset_word, read_program_byte, read_word_direct_page},
};

/// Calculate direct page address with X indexing
pub(crate) fn calculate_direct_page_x_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> (u16, u16) {
    let offset: u8 = read_offset_byte(cpu, bus);

    // X width: 8-bit in emulation mode, otherwise depends on X flag
    let x_index_u16: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.x as u8) as u16
    } else {
        cpu.registers.x
    };

    let direct_page = cpu.registers.d;
    let base_address = direct_page.wrapping_add(offset as u16);

    let target_address = if cpu.emulation_mode && (direct_page & 0x00FF) == 0 {
        // Emulation + page-aligned DP: wrap within page
        let wrapped = offset.wrapping_add(x_index_u16 as u8) as u16;
        (direct_page & 0xFF00) | wrapped
    } else {
        // Otherwise: full 16-bit add
        direct_page
            .wrapping_add(offset as u16)
            .wrapping_add(x_index_u16)
    };

    (base_address, target_address)
}

/// Calculate direct page address with Y indexing
pub(crate) fn calculate_direct_page_y_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset: u8 = read_offset_byte(cpu, bus);

    let y: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y as u8) as u16
    } else {
        cpu.registers.y
    };

    let direct_page = cpu.registers.d;

    if cpu.emulation_mode && (direct_page & 0x00FF) == 0 {
        // Emulation + page-aligned DP: wrap within page
        let wrapped = offset.wrapping_add(y as u8) as u16;
        (direct_page & 0xFF00) | wrapped
    } else {
        // Otherwise: full 16-bit add
        direct_page.wrapping_add(offset as u16).wrapping_add(y)
    }
}

/// Calculate direct page address
pub(crate) fn calculate_direct_page_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset = read_offset_byte(cpu, bus);
    cpu.registers.d.wrapping_add(offset as u16)
}

/// Calculate indirect indexed X address (indexed indirect)
pub(crate) fn calculate_indirect_page_x_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16, u16) {
    let offset: u8 = read_offset_byte(cpu, bus);

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

/// Calculate indirect indexed Y address (indirect indexed)
pub(crate) fn calculate_indirect_page_y_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16) {
    let offset = read_offset_byte(cpu, bus);
    let pointer_address = cpu.registers.d.wrapping_add(offset as u16);
    let base_address = read_word_direct_page(bus, pointer_address);

    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let address = base_address.wrapping_add(y);
    (base_address, address)
}

/// Calculate indirect address
pub(crate) fn calculate_indirect_page_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset: u8 = read_offset_byte(cpu, bus);
    let pointer_address: u16 = cpu.registers.d.wrapping_add(offset as u16);
    read_word_direct_page(bus, pointer_address)
}

/// Calculate stack relative address
pub(crate) fn calculate_stack_relative_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u16 {
    let offset: u8 = read_offset_byte(cpu, bus);
    cpu.registers.s.wrapping_add(offset as u16) // bank 0
}

/// Calculate absolute long address (24-bit)
pub(crate) fn calculate_absolute_long_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> u32 {
    // Reads ll, hh, bb from instruction stream using PBR
    let pc = cpu.registers.pc;

    let addr_low = read_program_byte(cpu, bus, pc.wrapping_add(1));
    let addr_mid = read_program_byte(cpu, bus, pc.wrapping_add(2));
    let addr_bank = read_program_byte(cpu, bus, pc.wrapping_add(3));

    ((addr_bank as u32) << 16) | ((addr_mid as u32) << 8) | (addr_low as u32)
}

/// Calculate stack relative indirect indexed Y address
pub(crate) fn calculate_stack_relative_indirect_y_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u16, u16, u16) {
    let offset: u8 = read_offset_byte(cpu, bus);

    // IMPORTANT: In emulation mode, SST-style traces treat stack accesses as if S is in $01xx
    // (linear 16-bit during execution), even if cpu.registers.s has a different high byte.
    let s_for_addressing: u16 = if cpu.emulation_mode {
        (STACK_START as u16) | (cpu.registers.s & 0x00FF)
    } else {
        cpu.registers.s
    };

    let pointer_address: u16 = s_for_addressing.wrapping_add(offset as u16);

    // Pointer is read from bank 0
    let base_address: u16 = read_word_direct_page(bus, pointer_address);

    // Y width: emulation forces 8-bit; native depends on X flag
    let y: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let effective: u16 = base_address.wrapping_add(y);

    (pointer_address, base_address, effective)
}

/// Dummy bus read used by SST-style traces for ($ss,S),Y.
/// In your failing case the trace shows this as reading pointer_address+1.
pub(crate) fn stack_relative_indirect_y_dummy_read<B: MemoryBus>(
    _cpu: &Cpu,
    bus: &mut B,
    pointer_address: u16,
) {
    // Stack is always bank 0
    let _ = bus.read(pointer_address.wrapping_add(1) as u32);
}

/// Calculate effective physical address for stack relative indirect Y
pub(crate) fn effective_phys_stack_relative_indirect_y(cpu: &Cpu, base_address: u16) -> u32 {
    // Y width: emulation forces 8-bit; native depends on X flag
    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y & 0x00FF) as u32
    } else {
        cpu.registers.y as u32
    };

    // IMPORTANT: 24-bit add so carry increments bank (matches SST traces)
    let base_phys = ((cpu.registers.db as u32) << 16) | (base_address as u32);
    (base_phys.wrapping_add(y)) & 0x00FF_FFFF
}

/// Read a byte using stack relative indirect Y addressing
pub(crate) fn read_data_byte_stack_relative_indirect_y<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
    base_address: u16,
) -> u8 {
    let phys = effective_phys_stack_relative_indirect_y(cpu, base_address);
    bus.read(phys)
}

/// Read a word using stack relative indirect Y addressing
pub(crate) fn read_data_word_stack_relative_indirect_y<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
    base_address: u16,
) -> u16 {
    let phys = effective_phys_stack_relative_indirect_y(cpu, base_address);
    let lo = bus.read(phys);
    let hi = bus.read((phys.wrapping_add(1)) & 0x00FF_FFFF);
    u16::from_le_bytes([lo, hi])
}

/// Check if indirect Y addressing needs an extra cycle
pub(crate) fn indirect_y_extra_cycle(cpu: &Cpu, base_address: u16, address16: u16) -> bool {
    let crossed = page_crossed(base_address, address16);

    if cpu.emulation_mode {
        // Emulation: extra cycle only on page cross
        crossed
    } else {
        // Native: if index is 16-bit (X=0), always an extra cycle;
        // if index is 8-bit (X=1), only on page cross.
        (!is_8bit_mode_x(cpu)) || crossed
    }
}

/// Calculate effective physical address for indirect Y addressing
pub(crate) fn effective_phys_indirect_y(cpu: &Cpu, base_address: u16) -> u32 {
    // Always compute effective as (DBR:base) + Y in 24-bit space (matches your traces)
    let y = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        (cpu.registers.y & 0x00FF) as u32
    } else {
        cpu.registers.y as u32
    };

    let base_phys = ((cpu.registers.db as u32) << 16) | (base_address as u32);
    (base_phys.wrapping_add(y)) & 0x00FF_FFFF
}

/// Calculate dummy read address for indirect Y addressing
pub(crate) fn dummy_phys_indirect_y(
    cpu: &Cpu,
    base_address: u16,
    address16: u16,
    effective_phys: u32,
) -> u32 {
    let crossed = page_crossed(base_address, address16);

    // What address gets used for the dummy read:
    // - On page cross, your traces show a "partial" address (derived from base high + effective low)
    // - Otherwise (native + X=0), the dummy is at the effective address itself (as in 11 n 427)
    if crossed {
        let dummy_addr16 = (base_address & 0xFF00) | (address16 & 0x00FF);
        (((cpu.registers.db as u32) << 16) | (dummy_addr16 as u32)) & 0x00FF_FFFF
    } else {
        effective_phys & 0x00FF_FFFF
    }
}

/// Read a byte using indirect Y addressing
pub(crate) fn read_data_byte_indirect_y<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
    base_address: u16,
    address16: u16,
) -> (u8, bool) {
    let extra = indirect_y_extra_cycle(cpu, base_address, address16);
    let eff = effective_phys_indirect_y(cpu, base_address);

    if extra {
        let dummy = dummy_phys_indirect_y(cpu, base_address, address16, eff);
        let _ = bus.read(dummy);
    }

    (bus.read(eff), extra)
}

/// Read a word using indirect Y addressing
pub(crate) fn read_data_word_indirect_y<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
    base_address: u16,
    address16: u16,
) -> (u16, bool) {
    let extra = indirect_y_extra_cycle(cpu, base_address, address16);
    let eff = effective_phys_indirect_y(cpu, base_address);

    if extra {
        let dummy = dummy_phys_indirect_y(cpu, base_address, address16, eff);
        let _ = bus.read(dummy);
    }

    let lo = bus.read(eff);
    let hi = bus.read((eff.wrapping_add(1)) & 0x00FF_FFFF);
    (u16::from_le_bytes([lo, hi]), extra)
}

/// Get absolute X indexed address
pub(crate) fn calculate_absolute_x_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> (u16, u16) {
    let base_address = read_offset_word(cpu, bus);
    let eff = base_address.wrapping_add(get_x_register_value(cpu));
    (base_address, eff)
}

/// Get absolute Y indexed address
pub(crate) fn calculate_absolute_y_address<B: MemoryBus>(cpu: &Cpu, bus: &mut B) -> (u16, u16) {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address.wrapping_add(get_y_register_value(cpu));

    (base_address, address)
}

pub(crate) fn calculate_absolute_long_x_address<B: MemoryBus>(
    cpu: &Cpu,
    bus: &mut B,
) -> (u32, u32) {
    // Reads ll, hh, bb from instruction stream using PBR
    let base_phys = calculate_absolute_long_address(cpu, bus) & 0x00FF_FFFF;

    // X width: emulation forces 8-bit; native depends on X flag
    let x = get_x_register_value(cpu) as u32;

    // Effective physical address in 24-bit space (carry wraps at 24-bit)
    let eff_phys = (base_phys.wrapping_add(x)) & 0x00FF_FFFF;

    (base_phys, eff_phys)
}

/// Get the value of the X register based on the current mode
pub(crate) fn get_x_register_value(cpu: &Cpu) -> u16 {
    if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.x & 0xFF
    } else {
        cpu.registers.x
    }
}

pub(crate) fn get_y_register_value(cpu: &Cpu) -> u16 {
    if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    }
}

/// Check if a page boundary was crossed
pub(crate) fn page_crossed(base_address: u16, target_address: u16) -> bool {
    (base_address & 0xFF00) != (target_address & 0xFF00)
}
