use crate::apu::{opcodes::get_direct_page_address, spc700::Spc700};

pub fn cmp_dp_imm(spc700: &mut Spc700) {
    let immediate = spc700.read_byte();
    let offset = spc700.read_byte() as u32;
    let dp_value = spc700.read(get_direct_page_address(spc700, offset));
    let result = dp_value.wrapping_sub(immediate);

    spc700.set_z(result);
    spc700.set_n(result);
    spc700.set_c(dp_value, immediate);
}

pub fn cmp_y_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_value = spc700.read(get_direct_page_address(spc700, offset));
    let result = spc700.registers.y.wrapping_sub(dp_value);

    spc700.set_z(result);
    spc700.set_n(result);
    spc700.set_c(spc700.registers.y, dp_value);
}
