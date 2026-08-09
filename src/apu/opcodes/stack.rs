use crate::apu::spc700::Spc700;

pub fn push_a(spc700: &mut Spc700) {
    spc700.push_byte(spc700.registers.a);
}

pub fn push_x(spc700: &mut Spc700) {
    spc700.push_byte(spc700.registers.x);
}

pub fn push_y(spc700: &mut Spc700) {
    spc700.push_byte(spc700.registers.y);
}

pub fn push_psw(spc700: &mut Spc700) {
    spc700.push_byte(spc700.registers.psw.0);
}

pub fn pop_a(spc700: &mut Spc700) {
    spc700.registers.a = spc700.pop_byte();
}

pub fn pop_x(spc700: &mut Spc700) {
    spc700.registers.x = spc700.pop_byte();
}

pub fn pop_y(spc700: &mut Spc700) {
    spc700.registers.y = spc700.pop_byte();
}

pub fn pop_psw(spc700: &mut Spc700) {
    spc700.registers.psw.0 = spc700.pop_byte();
}

pub fn call_abs(spc700: &mut Spc700) {
    let address = spc700.read_word();
    spc700.push_word(spc700.registers.pc);
    spc700.registers.pc = address;
}

pub fn tcall(spc700: &mut Spc700, n: u8) {
    spc700.push_word(spc700.registers.pc);
    let vector_addr = (0xFFDE_u16).wrapping_sub((n as u16) * 2) as u32;
    spc700.registers.pc = spc700.read_word_direct(vector_addr);
}

pub fn pcall(spc700: &mut Spc700) {
    let offset = spc700.read_byte();
    spc700.push_word(spc700.registers.pc);
    spc700.registers.pc = 0xFF00 | (offset as u16);
}

pub fn ret(spc700: &mut Spc700) {
    spc700.registers.pc = spc700.pop_word();
}

pub fn ret1(spc700: &mut Spc700) {
    spc700.registers.psw.0 = spc700.pop_byte();
    spc700.registers.pc = spc700.pop_word();
}

pub fn brk(spc700: &mut Spc700) {
    spc700.push_word(spc700.registers.pc);
    spc700.push_byte(spc700.registers.psw.0);
    spc700.registers.psw.set_brk(true);
    spc700.registers.psw.set_interrupt_enable(false);
    spc700.registers.pc = spc700.read_word_direct(0xFFDE);
}
