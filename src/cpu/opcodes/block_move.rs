use crate::{
    cpu::{
        Cpu,
        opcodes::{
            increment_program_counter, is_8bit_mode_m, is_8bit_mode_x, read_byte, read_offset_byte,
        },
    },
    memory::MemoryBus,
};

pub fn mvp<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    // IMPORTANT: MVP operand order is: dest bank, source bank
    // SST trace shows reads at PC, PC+1, PC+2 repeatedly.
    let pc = cpu.registers.pc;

    // We'll read operands using program space reads so the bus trace matches.
    let dest_bank = read_byte(cpu, bus, pc.wrapping_add(1));
    let source_bank = read_byte(cpu, bus, pc.wrapping_add(2));

    let mut cycles: u32 = 0;

    loop {
        // --- Per-iteration "instruction fetch" dummy reads (matches SST) ---
        let _ = read_byte(cpu, bus, pc); // opcode
        let _ = read_byte(cpu, bus, pc.wrapping_add(1)); // dest bank
        let _ = read_byte(cpu, bus, pc.wrapping_add(2)); // src bank
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
    (cycles as u8)
}

// MVN - Block Move Negative
// Copies a block of memory from source to destination, moving backward through memory.
// Uses X as source address, Y as destination address, A as byte count - 1.
// Decrements X and Y after each byte. Decrements A.
// If A != $FFFF after transfer, repeats (doesn't advance PC).
// Operands specify source bank and destination bank.
pub fn mvn<B: MemoryBus>(cpu: &mut Cpu, bus: &mut B) -> u8 {
    let dest_bank = read_offset_byte(cpu, bus);
    let source_bank = read_offset_byte(cpu, bus);

    // Read byte from source bank:X
    let source_address = ((source_bank as u32) << 16) | (cpu.registers.x as u32);
    let byte = bus.read(source_address);

    // Write byte to dest bank:Y
    let dest_address = ((dest_bank as u32) << 16) | (cpu.registers.y as u32);
    bus.write(dest_address, byte);

    // Update registers
    cpu.registers.x = cpu.registers.x.wrapping_sub(1);
    cpu.registers.y = cpu.registers.y.wrapping_sub(1);
    cpu.registers.a = cpu.registers.a.wrapping_sub(1);

    // Set DB to destination bank
    cpu.registers.db = dest_bank;

    // If A is not $FFFF, repeat (don't advance PC)
    // Otherwise, advance PC by 3 to move to next instruction
    if cpu.registers.a != 0xFFFF {
        // Don't increment PC, will execute MVN again
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(3);
    }

    7
}
