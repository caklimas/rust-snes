use crate::apu::spc700::Spc700;

pub fn inc_a(spc700: &mut Spc700) {
    spc700.registers.a = spc700.registers.a.wrapping_add(1);
    spc700.set_n(spc700.registers.a);
    spc700.set_z(spc700.registers.a);
}

pub fn inc_x(spc700: &mut Spc700) {
    spc700.registers.x = spc700.registers.x.wrapping_add(1);
    spc700.set_n(spc700.registers.x);
    spc700.set_z(spc700.registers.x);
}

pub fn inc_y(spc700: &mut Spc700) {
    spc700.registers.y = spc700.registers.y.wrapping_add(1);
    spc700.set_n(spc700.registers.y);
    spc700.set_z(spc700.registers.y);
}

pub fn inc_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let value = spc700.read(dp_address).wrapping_add(1);
    spc700.write(dp_address, value);
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn inc_dp_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let address = spc700.get_direct_page_address((offset + x) & 0xFF);
    let value = spc700.read(address).wrapping_add(1);
    spc700.write(address, value);
    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn inc_abs(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address).wrapping_add(1);
    spc700.write(address, value);
    spc700.set_n(value);
    spc700.set_z(value);
}
