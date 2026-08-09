use crate::apu::spc700::Spc700;

fn compare(spc700: &mut Spc700, a: u8, b: u8) {
    let result = a.wrapping_sub(b);
    spc700.set_z(result);
    spc700.set_n(result);
    spc700.set_c(a, b);
}

pub fn cmp_x_imm(spc700: &mut Spc700) {
    let imm = spc700.read_byte();
    compare(spc700, spc700.registers.x, imm);
}

pub fn cmp_x_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(offset));
    compare(spc700, spc700.registers.x, value);
}

pub fn cmp_x_abs(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    compare(spc700, spc700.registers.x, value);
}

pub fn cmp_y_imm(spc700: &mut Spc700) {
    let imm = spc700.read_byte();
    compare(spc700, spc700.registers.y, imm);
}

pub fn cmp_y_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(offset));
    compare(spc700, spc700.registers.y, value);
}

pub fn cmp_y_abs(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    compare(spc700, spc700.registers.y, value);
}
