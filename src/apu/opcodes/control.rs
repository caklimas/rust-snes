use crate::apu::spc700::Spc700;

pub fn nop(_spc700: &mut Spc700) {}

pub fn sleep(spc700: &mut Spc700) {
    spc700.registers.pc = spc700.registers.pc.wrapping_sub(1);
}

pub fn stop(spc700: &mut Spc700) {
    spc700.registers.pc = spc700.registers.pc.wrapping_sub(1);
}

pub fn clrp(spc700: &mut Spc700) {
    spc700.registers.psw.set_direct_page(false);
}

pub fn setp(spc700: &mut Spc700) {
    spc700.registers.psw.set_direct_page(true);
}

pub fn ei(spc700: &mut Spc700) {
    spc700.registers.psw.set_interrupt_enable(true);
}

pub fn di(spc700: &mut Spc700) {
    spc700.registers.psw.set_interrupt_enable(false);
}

pub fn xcn(spc700: &mut Spc700) {
    let a = spc700.registers.a;
    spc700.registers.a = a.rotate_left(4);
    spc700.set_n(spc700.registers.a);
    spc700.set_z(spc700.registers.a);
}

pub fn daa(spc700: &mut Spc700) {
    let mut a = spc700.registers.a;
    if spc700.registers.psw.carry() || a > 0x99 {
        a = a.wrapping_add(0x60);
        spc700.registers.psw.set_carry(true);
    }
    if spc700.registers.psw.half_carry() || (a & 0x0F) > 0x09 {
        a = a.wrapping_add(0x06);
    }
    spc700.registers.a = a;
    spc700.set_n(a);
    spc700.set_z(a);
}

pub fn das(spc700: &mut Spc700) {
    let mut a = spc700.registers.a;
    if !spc700.registers.psw.carry() || a > 0x99 {
        a = a.wrapping_sub(0x60);
        spc700.registers.psw.set_carry(false);
    }
    if !spc700.registers.psw.half_carry() || (a & 0x0F) > 0x09 {
        a = a.wrapping_sub(0x06);
    }
    spc700.registers.a = a;
    spc700.set_n(a);
    spc700.set_z(a);
}

pub fn tset1(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    let a = spc700.registers.a;
    let test = a.wrapping_sub(value);
    spc700.set_n(test);
    spc700.set_z(test);
    spc700.write(address, value | a);
}

pub fn tclr1(spc700: &mut Spc700) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    let a = spc700.registers.a;
    let test = a.wrapping_sub(value);
    spc700.set_n(test);
    spc700.set_z(test);
    spc700.write(address, value & !a);
}
