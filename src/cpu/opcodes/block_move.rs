use crate::{
    cpu::{Cpu, opcodes::read_offset_byte},
    memory::bus::Bus,
};

// MVP - Block Move Positive
// Copies a block of memory from source to destination, moving forward through memory.
// Uses X as source address, Y as destination address, A as byte count - 1.
// Increments X and Y after each byte. Decrements A.
// If A != $FFFF after transfer, repeats (doesn't advance PC).
// Operands specify source bank and destination bank.
pub fn mvp(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let dest_bank = read_offset_byte(cpu, bus) as u8;
    let source_bank = read_offset_byte(cpu, bus) as u8;

    // Read byte from source bank:X
    let source_address = ((source_bank as u32) << 16) | (cpu.registers.x as u32);
    let byte = bus.read(source_address);

    // Write byte to dest bank:Y
    let dest_address = ((dest_bank as u32) << 16) | (cpu.registers.y as u32);
    bus.write(dest_address, byte);

    // Update registers
    cpu.registers.x = cpu.registers.x.wrapping_add(1);
    cpu.registers.y = cpu.registers.y.wrapping_add(1);
    cpu.registers.a = cpu.registers.a.wrapping_sub(1);

    // Set DB to destination bank
    cpu.registers.db = dest_bank;

    // If A is not $FFFF, repeat (don't advance PC)
    // Otherwise, advance PC by 3 to move to next instruction
    if cpu.registers.a != 0xFFFF {
        // Don't increment PC, will execute MVP again
    } else {
        cpu.registers.pc = cpu.registers.pc.wrapping_add(3);
    }

    7
}

// MVN - Block Move Negative
// Copies a block of memory from source to destination, moving backward through memory.
// Uses X as source address, Y as destination address, A as byte count - 1.
// Decrements X and Y after each byte. Decrements A.
// If A != $FFFF after transfer, repeats (doesn't advance PC).
// Operands specify source bank and destination bank.
pub fn mvn(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let dest_bank = read_offset_byte(cpu, bus) as u8;
    let source_bank = read_offset_byte(cpu, bus) as u8;

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
