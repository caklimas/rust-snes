use crate::apu::spc700::Spc700;

pub fn bne_rel(spc700: &mut Spc700) {
    let value = read_offset(spc700);
    if !spc700.registers.psw.zero() {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(value as u16);
    }
}

pub fn bra_rel(spc700: &mut Spc700) {
    let value = read_offset(spc700);
    spc700.registers.pc = spc700.registers.pc.wrapping_add(value as u16);
}

pub fn bpl_rel(spc700: &mut Spc700) {
    let value = read_offset(spc700);
    if !spc700.registers.psw.negative() {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(value as u16);
    }
}

fn read_offset(spc700: &mut Spc700) -> i16 {
    (spc700.read_byte() as i8) as i16
}
