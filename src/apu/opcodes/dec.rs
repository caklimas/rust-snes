use crate::apu::spc700::Spc700;

pub fn dec_x(spc700: &mut Spc700) {
    let value = spc700.registers.x.wrapping_sub(1);

    spc700.registers.x = value;

    spc700.set_n(value);
    spc700.set_z(value);
}
