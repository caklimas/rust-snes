use crate::apu::spc700::Spc700;

pub fn inc_y(spc700: &mut Spc700) {
    let value = spc700.registers.y.wrapping_add(1);

    spc700.registers.y = value;

    spc700.set_n(value);
    spc700.set_z(value);
}

pub fn inc_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let dp_value = spc700.read(dp_address).wrapping_add(1);

    spc700.write(dp_address, dp_value);

    spc700.set_n(dp_value);
    spc700.set_z(dp_value);
}
