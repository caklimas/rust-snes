use crate::apu::{opcodes::get_direct_page_address, spc700::Spc700};

pub fn mov_x_imm(spc700: &mut Spc700) {
    let value = spc700.read_byte();
    spc700.registers.x = value;

    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_sp_x(spc700: &mut Spc700) {
    spc700.registers.sp = spc700.registers.x;
}

pub fn mov_a_imm(spc700: &mut Spc700) {
    let value = spc700.read_byte();
    spc700.registers.a = value;

    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn mov_ind_x_a(spc700: &mut Spc700) {
    spc700.write(
        get_direct_page_address(spc700, spc700.registers.x as u32),
        spc700.registers.a,
    );
}

pub fn mov_a_y(spc700: &mut Spc700) {
    spc700.registers.a = spc700.registers.y;

    spc700.set_n(spc700.registers.y);
    spc700.set_z(spc700.registers.y);
}

pub fn mov_x_a(spc700: &mut Spc700) {
    spc700.registers.x = spc700.registers.a;

    spc700.set_n(spc700.registers.a);
    spc700.set_z(spc700.registers.a);
}
