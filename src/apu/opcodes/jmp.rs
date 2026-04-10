use crate::apu::spc700::Spc700;

pub fn jmp_abs_x(spc700: &mut Spc700) {
    let abs_address = spc700.read_word() as u32;
    let address = abs_address.wrapping_add(spc700.registers.x as u32);

    spc700.registers.pc = spc700.read_word_direct(address);
}
