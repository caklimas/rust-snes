use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_absolute_long_address, calculate_direct_page_address,
            calculate_direct_page_x_address, calculate_indirect_page_address,
            calculate_indirect_page_x_address, calculate_indirect_page_y_address,
            calculate_stack_relative_indirect_y_address, direct_page_low_is_zero,
            dummy_phys_indirect_y, effective_phys_indirect_y, get_address_absolute_long_x,
            get_x_register_value, get_y_register_value, increment_program_counter,
            indirect_y_extra_cycle, is_8bit_mode_m, is_8bit_mode_x, page_crossed, read_data_byte,
            read_data_byte_indirect_y, read_data_byte_stack_relative_indirect_y, read_data_word,
            read_data_word_indirect_y, read_data_word_stack_relative_indirect_y,
            read_long_pointer_direct_page_wrapped, read_offset_byte, read_offset_word,
            read_phys_byte, read_phys_word, read_word_direct_page, set_nz_flags_u8,
            set_nz_flags_u16, stack_relative_indirect_y_dummy_read,
        },
    },
    memory::MemoryBus,
};

// EOR - Exclusive OR with Accumulator
// Performs a bitwise XOR between the accumulator and a value from memory, storing the result in the accumulator.
// Sets N and Z flags based on the result. Commonly used for toggling specific bits or comparing bit patterns.

pub fn eor_immediate<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pc_increment, cycles) = if is_8bit_mode_m(cpu) {
        let value = read_offset_byte(cpu, bus);
        perform_eor_u8(cpu, value);
        (2, 2)
    } else {
        let value = read_offset_word(cpu, bus);
        perform_eor_u16(cpu, value);
        (3, 3)
    };

    increment_program_counter(cpu, pc_increment);
    cycles
}

pub fn eor_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let source_address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(source_address as u32);
        perform_eor_u8(cpu, value);
        3
    } else {
        let value = read_word_direct_page(bus, source_address);
        perform_eor_u16(cpu, value);
        4
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus); // 16-bit operand from instruction stream

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address); // <-- DBR:addr
        perform_eor_u8(cpu, value);
        4
    } else {
        let value = read_data_word(cpu, bus, address); // <-- DBR:addr (word)
        perform_eor_u16(cpu, value);
        5
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn eor_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        perform_eor_u8(cpu, value);
        4
    } else {
        let value = read_word_direct_page(bus, address);
        perform_eor_u16(cpu, value);
        5
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let x: u16 = get_x_register_value(cpu);
    let address16 = base_address.wrapping_add(x);

    let base_phys: u32 = ((cpu.registers.db as u32) << 16) | (base_address as u32);
    let effective_phys: u32 = base_phys.wrapping_add(x as u32) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) { 4 } else { 5 };

    let crossed = page_crossed(base_address, address16);
    let extra = crossed || (!cpu.emulation_mode && !is_8bit_mode_x(cpu));

    if extra {
        let dummy_phys = if crossed {
            let dummy_addr16 = (base_address & 0xFF00) | (address16 & 0x00FF);
            (((cpu.registers.db as u32) << 16) | (dummy_addr16 as u32)) & 0x00FF_FFFF
        } else {
            effective_phys
        };

        let _ = bus.read(dummy_phys);
        cycles += 1;
    }

    if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, effective_phys);
        perform_eor_u8(cpu, value);
    } else {
        let value = read_phys_word(bus, effective_phys);
        perform_eor_u16(cpu, value);
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn eor_absolute_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);

    let y = get_y_register_value(cpu);
    let address16 = base_address.wrapping_add(y);

    let base_phys: u32 = ((cpu.registers.db as u32) << 16) | (base_address as u32);
    let effective_phys: u32 = base_phys.wrapping_add(y as u32) & 0x00FF_FFFF;

    let mut cycles = if is_8bit_mode_m(cpu) { 4 } else { 5 };

    let crossed = page_crossed(base_address, address16);
    let extra = crossed || (!cpu.emulation_mode && !is_8bit_mode_x(cpu));

    if extra {
        let dummy_phys = if crossed {
            let dummy_addr16 = (base_address & 0xFF00) | (address16 & 0x00FF);
            (((cpu.registers.db as u32) << 16) | (dummy_addr16 as u32)) & 0x00FF_FFFF
        } else {
            effective_phys
        };

        let _ = bus.read(dummy_phys);
        cycles += 1;
    }

    if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, effective_phys);
        perform_eor_u8(cpu, value);
    } else {
        let value = read_phys_word(bus, effective_phys);
        perform_eor_u16(cpu, value);
    }

    increment_program_counter(cpu, 3);
    cycles
}

pub fn eor_indirect_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, _, address) = calculate_indirect_page_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        perform_eor_u8(cpu, value);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        perform_eor_u16(cpu, value);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (base_addr, addr16) = calculate_indirect_page_y_address(cpu, bus);

    // Compute the true 24-bit effective physical address for (dp),Y
    let effective_phys = effective_phys_indirect_y(cpu, base_addr);

    let mut cycles = if is_8bit_mode_m(cpu) {
        // Prefer your helper if you have it; otherwise bus.read(effective_phys) is fine too.
        let (value, _) = read_data_byte_indirect_y(cpu, bus, base_addr, addr16);
        perform_eor_u8(cpu, value);
        5
    } else {
        let (value, _) = read_data_word_indirect_y(cpu, bus, base_addr, addr16);
        perform_eor_u16(cpu, value);
        6
    };

    // Direct Page penalty (+1 if D low byte != 0)
    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    // Indirect,Y extra cycle + dummy read
    if indirect_y_extra_cycle(cpu, base_addr, addr16) {
        let dummy = dummy_phys_indirect_y(cpu, base_addr, addr16, effective_phys);
        let _ = bus.read(dummy);
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_indirect<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // Returns the 16-bit effective address from (dp) pointer read in bank 0
    let address16 = calculate_indirect_page_address(cpu, bus);

    // Base cycles for EOR (dp): 5 (m=1) or 6 (m=0)
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address16); // <-- DBR:addr
        perform_eor_u8(cpu, value);
        5
    } else {
        let value = read_data_word(cpu, bus, address16); // <-- DBR:addr
        perform_eor_u16(cpu, value);
        6
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_stack_relative<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let offset = read_offset_byte(cpu, bus);

    let s_for_addressing: u16 = if cpu.emulation_mode {
        0x0100 | (cpu.registers.s & 0x00FF)
    } else {
        cpu.registers.s
    };

    let addr = s_for_addressing.wrapping_add(offset as u16);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(addr as u32);
        perform_eor_u8(cpu, value);
        4
    } else {
        let lo = bus.read(addr as u32);
        let hi = bus.read(addr.wrapping_add(1) as u32);
        perform_eor_u16(cpu, u16::from_le_bytes([lo, hi]));
        5
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_direct_indirect_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // dp operand + D -> pointer location (in bank 0)
    let dp_ptr_addr = calculate_direct_page_address(cpu, bus);

    // Read 24-bit effective address from direct page (bank 0), with your emu wrapping behavior
    let effective_phys = read_long_pointer_direct_page_wrapped(cpu, bus, dp_ptr_addr);

    // Base cycles for EOR [dp] is 6
    let mut cycles: u8 = 6;

    // +1 if accumulator is 16-bit (m=0)
    // +1 if D.l != 0 (direct page register low byte non-zero)
    if !is_8bit_mode_m(cpu) {
        cycles += 1;
    }
    if (cpu.registers.d & 0x00FF) != 0 {
        cycles += 1;
    }

    if is_8bit_mode_m(cpu) {
        let m = bus.read(effective_phys);
        let a_lo = (cpu.registers.a & 0x00FF) as u8;
        let result = a_lo ^ m;

        cpu.registers.a = (cpu.registers.a & 0xFF00) | result as u16;
        set_nz_flags_u8(cpu, result);
    } else {
        // 16-bit memory read from 24-bit physical address (little-endian)
        let lo = bus.read(effective_phys) as u16;
        let hi = bus.read(effective_phys.wrapping_add(1)) as u16;
        let m = lo | (hi << 8);

        let result = cpu.registers.a ^ m;
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_absolute_long<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // 24-bit address is encoded in the instruction stream (PBR:PC+1..+3)
    let phys_addr = calculate_absolute_long_address(cpu, bus);

    // EOR long: 5 cycles (+1 if m=0 because 16-bit acc reads a word)
    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, phys_addr);
        perform_eor_u8(cpu, value);
        5
    } else {
        let value = read_phys_word(bus, phys_addr);
        perform_eor_u16(cpu, value);
        6
    };

    // opcode + 3-byte operand
    increment_program_counter(cpu, 4);
    cycles
}

pub fn eor_stack_relative_indirect_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (pointer_address, base_address, _effective16) =
        calculate_stack_relative_indirect_y_address(cpu, bus);

    stack_relative_indirect_y_dummy_read(cpu, bus, pointer_address);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte_stack_relative_indirect_y(cpu, bus, base_address);
        perform_eor_u8(cpu, value);
        7
    } else {
        let value = read_data_word_stack_relative_indirect_y(cpu, bus, base_address);
        perform_eor_u16(cpu, value);
        8
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_direct_indirect_long_y<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dp_addr = calculate_direct_page_address(cpu, bus);

    let base_phys = read_long_pointer_direct_page_wrapped(cpu, bus, dp_addr);

    let y: u16 = if cpu.emulation_mode || is_8bit_mode_x(cpu) {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let effective_phys = base_phys.wrapping_add(y as u32) & 0x00FF_FFFF;

    let mut cycles: u8 = 6;
    if !is_8bit_mode_m(cpu) {
        cycles += 1;
    }
    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, effective_phys);
        perform_eor_u8(cpu, value);
    } else {
        let value = read_phys_word(bus, effective_phys);
        perform_eor_u16(cpu, value);
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn eor_absolute_long_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, effective_phys) = get_address_absolute_long_x(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) { 5 } else { 6 };

    if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, effective_phys);
        let a_lo = (cpu.registers.a & 0x00FF) as u8;
        let result = a_lo ^ value;

        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
    } else {
        let value = read_phys_word(bus, effective_phys);
        let result = cpu.registers.a ^ value;

        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
    }

    increment_program_counter(cpu, 4);
    cycles
}

fn perform_eor_u8(cpu: &mut Cpu, value: u8) {
    let result = (cpu.registers.a & 0xFF) ^ (value as u16);
    cpu.registers.a = (cpu.registers.a & 0xFF00) | result;
    set_nz_flags_u8(cpu, result as u8);
}

fn perform_eor_u16(cpu: &mut Cpu, value: u16) {
    let result = cpu.registers.a ^ value;
    cpu.registers.a = result;
    set_nz_flags_u16(cpu, result);
}
