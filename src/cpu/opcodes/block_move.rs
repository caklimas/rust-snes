use crate::{
    cpu::{
        Cpu,
        opcodes::{
            get_x_register_value, increment_program_counter, is_8bit_mode_x, read_offset_byte,
            read_program_byte,
        },
    },
    memory::MemoryBus,
};

pub fn mvp<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // IMPORTANT: MVP operand order is: dest bank, source bank
    // SST trace shows reads at PC, PC+1, PC+2 repeatedly.
    let pc = cpu.registers.pc;

    // We'll read operands using program space reads so the bus trace matches.
    let dest_bank = read_program_byte(cpu, bus, pc.wrapping_add(1));
    let source_bank = read_program_byte(cpu, bus, pc.wrapping_add(2));

    let mut cycles: u32 = 0;

    loop {
        // --- Per-iteration "instruction fetch" dummy reads (matches SST) ---
        let _ = read_program_byte(cpu, bus, pc); // opcode
        let _ = read_program_byte(cpu, bus, pc.wrapping_add(1)); // dest bank
        let _ = read_program_byte(cpu, bus, pc.wrapping_add(2)); // src bank
        cycles += 3;

        // --- Actual move ---
        let source_phys = ((source_bank as u32) << 16) | (cpu.registers.x as u32);
        let value = bus.read(source_phys);
        cycles += 1;

        let dest_phys = ((dest_bank as u32) << 16) | (cpu.registers.y as u32);
        bus.write(dest_phys, value);
        cycles += 1;

        // --- Two "idle" cycles (SST shows these as reads from the dest address with null data) ---
        let _ = bus.read(dest_phys);
        let _ = bus.read(dest_phys);
        cycles += 2;

        // --- Register updates ---
        cpu.registers.x = cpu.registers.x.wrapping_sub(1);
        cpu.registers.y = cpu.registers.y.wrapping_sub(1);

        // The block move counter is the 16-bit C accumulator (what SST calls "a")
        cpu.registers.a = cpu.registers.a.wrapping_sub(1);

        // DBR becomes destination bank each iteration (SST expects final dbr == dest_bank)
        cpu.registers.db = dest_bank;

        // In emulation (and/or X flag), index registers are 8-bit “visible”.
        // SST expects high byte to be zeroed when X=1/E=1.
        if cpu.emulation_mode || is_8bit_mode_x(cpu) {
            cpu.registers.x &= 0x00FF;
            cpu.registers.y &= 0x00FF;
        }

        // Termination condition for block moves:
        // stop when the counter underflows to $FFFF.
        if cpu.registers.a == 0xFFFF {
            break;
        }
    }

    increment_program_counter(cpu, 2);

    // Your core returns u8 cycles; clamp if needed.
    // For SST-style cases this stays small.
    cycles as u8
}

pub fn mvn<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // Operands are in program space (PBR:PC+1 / PBR:PC+2)
    let dest_bank = read_offset_byte(cpu, bus);
    let src_bank = read_program_byte(cpu, bus, cpu.registers.pc.wrapping_add(2));

    // MVN sets DBR to destination bank
    cpu.registers.db = dest_bank;

    // X width: emulation forces 8-bit; native depends on X flag
    let x_is_8 = cpu.emulation_mode || is_8bit_mode_x(cpu);
    let x = get_x_register_value(cpu);

    // Y width matches X width rules in your addressing helpers
    let y: u16 = if x_is_8 {
        cpu.registers.y & 0x00FF
    } else {
        cpu.registers.y
    };

    let src_phys = (((src_bank as u32) << 16) | (x as u32)) & 0x00FF_FFFF;
    let dst_phys = (((dest_bank as u32) << 16) | (y as u32)) & 0x00FF_FFFF;

    let value = bus.read(src_phys);
    bus.write(dst_phys, value);

    // X++, Y++
    if x_is_8 {
        cpu.registers.x = (cpu.registers.x & 0xFF00) | ((x as u8).wrapping_add(1) as u16);
        cpu.registers.y = (cpu.registers.y & 0xFF00) | ((y as u8).wrapping_add(1) as u16);
    } else {
        cpu.registers.x = x.wrapping_add(1);
        cpu.registers.y = y.wrapping_add(1);
    }

    // A-- (16-bit counter)
    cpu.registers.a = cpu.registers.a.wrapping_sub(1);

    // When A underflows to 0xFFFF, the instruction finishes and PC advances
    if cpu.registers.a == 0xFFFF {
        increment_program_counter(cpu, 3);
    }

    7
}
