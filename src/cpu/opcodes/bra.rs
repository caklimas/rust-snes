use crate::{
    cpu::{
        Cpu,
        opcodes::{increment_program_counter, read_byte, read_offset_byte, read_offset_word},
        processor_status::ProcessorStatus,
    },
    memory::{addresses, bus::Bus},
};

pub fn bra_relative(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let displacement = read_offset_byte(cpu, bus);
    let signed_displacement = if displacement > 127 {
        displacement - 256
    } else {
        displacement
    };

    let target_address = cpu.registers.pc + 2 + signed_displacement;
    let page_crossed = is_page_crossed(cpu, target_address);

    cpu.registers.pc = target_address;

    if page_crossed { 4 } else { 3 }
}

pub fn bra_relative_long(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    let displacement = read_offset_word(cpu, bus) as i16;
    let target_address = (cpu.registers.pc + 3).wrapping_add(displacement as u16);

    cpu.registers.pc = target_address;

    4
}

pub fn beq(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(cpu, bus, !cpu.registers.p.contains(ProcessorStatus::ZERO))
}

pub fn bne(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(cpu, bus, cpu.registers.p.contains(ProcessorStatus::ZERO))
}

pub fn bcc(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(cpu, bus, !cpu.registers.p.contains(ProcessorStatus::CARRY))
}

pub fn bcs(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(cpu, bus, cpu.registers.p.contains(ProcessorStatus::CARRY))
}

pub fn bmi(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(
        cpu,
        bus,
        cpu.registers.p.contains(ProcessorStatus::NEGATIVE),
    )
}

pub fn bpl(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(
        cpu,
        bus,
        !cpu.registers.p.contains(ProcessorStatus::NEGATIVE),
    )
}

pub fn bvc(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(
        cpu,
        bus,
        !cpu.registers.p.contains(ProcessorStatus::OVERFLOW),
    )
}

pub fn bvs(cpu: &mut Cpu, bus: &mut Bus) -> u8 {
    branch_conditional(
        cpu,
        bus,
        cpu.registers.p.contains(ProcessorStatus::OVERFLOW),
    )
}

fn branch_conditional(cpu: &mut Cpu, bus: &mut Bus, flag_conditional: bool) -> u8 {
    let cycles = 2;
    if flag_conditional {
        increment_program_counter(cpu, 2);
        return cycles;
    }

    let displacement = read_offset_byte(cpu, bus) as i8;
    let target_address = (cpu.registers.pc + 2).wrapping_add(displacement as u16);

    cpu.registers.pc = target_address;

    if is_page_crossed(cpu, target_address) {
        cycles + 1
    } else {
        cycles
    }
}

fn is_page_crossed(cpu: &Cpu, address: u16) -> bool {
    ((cpu.registers.pc + 2) & 0xFF00) != (address & 0xFF00)
}
