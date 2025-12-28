use crate::{
    cpu::{
        Cpu,
        opcodes::{
            calculate_direct_page_address, calculate_direct_page_x_address,
            direct_page_low_is_zero, get_address_absolute_x_data_physical, get_x_register_value,
            increment_program_counter, is_8bit_mode_m, read_byte, read_data_byte, read_data_word,
            read_offset_word, read_phys_byte, read_phys_word, read_word, read_word_direct_page,
            set_nz_flags_u8, set_nz_flags_u16, write_byte, write_byte_direct_page, write_word,
            write_word_direct_page,
        },
        processor_status::ProcessorStatus,
    },
    memory::MemoryBus,
};

// ASL - Arithmetic Shift Left
// Shifts all bits left by one position. Bit 0 is filled with 0, and the original bit 7/15 goes into the carry flag.
// Sets N, Z, and C flags. Effectively multiplies the value by 2.

pub fn asl_accumulator<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x80 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn asl_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte_direct_page(bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word_direct_page(bus, address);
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word_direct_page(bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn asl_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn asl_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte_direct_page(bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word_direct_page(bus, address);
        let result = value << 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word_direct_page(bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn asl_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_base, _eff16, phys) = get_address_absolute_x_data_physical(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_phys_byte(bus, phys);
        let result = value << 1;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x80) != 0);

        // RMW dummy write + final write (matches your trace)
        bus.write(phys, value);
        bus.write(phys, result);

        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_phys_word(bus, phys);
        let result = value << 1;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x8000) != 0);

        // dummy write old word
        bus.write(phys, (value & 0x00FF) as u8);
        bus.write((phys.wrapping_add(1)) & 0x00FF_FFFF, (value >> 8) as u8);

        // final write new word
        bus.write(phys, (result & 0x00FF) as u8);
        bus.write((phys.wrapping_add(1)) & 0x00FF_FFFF, (result >> 8) as u8);

        set_nz_flags_u16(cpu, result);
        9
    };

    increment_program_counter(cpu, 3);
    cycles
}

// LSR - Logical Shift Right
// Shifts all bits right by one position. Bit 7/15 is filled with 0, and the original bit 0 goes into the carry flag.
// Sets N (always 0), Z, and C flags. Effectively divides the value by 2 (unsigned).

pub fn lsr_accumulator<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn lsr_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word_direct_page(bus, address);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn lsr_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn lsr_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word_direct_page(bus, address);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn lsr_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let result = value >> 1;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// ROL - Rotate Left
// Shifts all bits left by one position. The carry flag goes into bit 0, and bit 7/15 goes into the carry flag.
// Sets N, Z, and C flags. Used for multi-byte shifts and bit manipulation.

pub fn rol_accumulator<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x80 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn rol_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);

        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x80) != 0);

        // RMW dummy write + final write (matches your trace)
        write_byte_direct_page(bus, address, value);
        write_byte_direct_page(bus, address, result);

        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word_direct_page(bus, address);

        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x8000) != 0);

        // If you want to match traces for 16-bit too, do dummy write of old word here as well.
        write_word_direct_page(bus, address, value);
        write_word_direct_page(bus, address, result);

        set_nz_flags_u16(cpu, result);
        7
    };

    // +1 cycle if D.l != 0 for DP addressing
    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn rol_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_data_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x80) != 0);

        write_byte(cpu, bus, address, value);
        write_byte(cpu, bus, address, result);

        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_data_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | (carry_in as u16);

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x8000) != 0);

        write_word(cpu, bus, address, value);
        write_word(cpu, bus, address, result);

        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn rol_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);

    let mut cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x80) != 0);

        write_byte_direct_page(bus, address, value);
        write_byte_direct_page(bus, address, result);

        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word_direct_page(bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;

        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, (value & 0x8000) != 0);

        write_word_direct_page(bus, address, value);
        write_word_direct_page(bus, address, result);

        set_nz_flags_u16(cpu, result);
        8
    };

    if !direct_page_low_is_zero(cpu) {
        cycles += 1;
    }

    increment_program_counter(cpu, 2);
    cycles
}

pub fn rol_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x80 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            1
        } else {
            0
        };
        let result = (value << 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x8000 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}

// ROR - Rotate Right
// Shifts all bits right by one position. The carry flag goes into bit 7/15, and bit 0 goes into the carry flag.
// Sets N, Z, and C flags. Used for multi-byte shifts and bit manipulation.

pub fn ror_accumulator<B: MemoryBus>(cpu: &mut Cpu, _bus: &mut B) -> u8 {
    let cycles = if is_8bit_mode_m(cpu) {
        let value = (cpu.registers.a & 0xFF) as u8;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x80
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        cpu.registers.a = (cpu.registers.a & 0xFF00) | (result as u16);
        set_nz_flags_u8(cpu, result);
        2
    } else {
        let value = cpu.registers.a;
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x8000
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        cpu.registers.a = result;
        set_nz_flags_u16(cpu, result);
        2
    };

    increment_program_counter(cpu, 1);
    cycles
}

pub fn ror_direct<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = calculate_direct_page_address(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x80
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        5
    } else {
        let value = read_word_direct_page(bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x8000
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        6
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ror_absolute<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let address = read_offset_word(cpu, bus);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x80
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x8000
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 3);
    cycles
}

pub fn ror_direct_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let (_, address) = calculate_direct_page_x_address(cpu, bus);
    let cycles = if is_8bit_mode_m(cpu) {
        let value = bus.read(address as u32);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x80
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        6
    } else {
        let value = read_word_direct_page(bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x8000
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        7
    };

    increment_program_counter(cpu, 2);
    cycles
}

pub fn ror_absolute_x<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let base_address = read_offset_word(cpu, bus);
    let address = base_address + get_x_register_value(cpu);

    let cycles = if is_8bit_mode_m(cpu) {
        let value = read_byte(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x80
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x01 != 0);
        write_byte(cpu, bus, address, result);
        set_nz_flags_u8(cpu, result);
        7
    } else {
        let value = read_word(cpu, bus, address);
        let carry_in = if cpu.registers.p.contains(ProcessorStatus::CARRY) {
            0x8000
        } else {
            0
        };
        let result = (value >> 1) | carry_in;
        cpu.registers
            .p
            .set(ProcessorStatus::CARRY, value & 0x0001 != 0);
        write_word(cpu, bus, address, result);
        set_nz_flags_u16(cpu, result);
        8
    };

    increment_program_counter(cpu, 3);
    cycles
}
