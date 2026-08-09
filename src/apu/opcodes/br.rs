use crate::apu::spc700::Spc700;

fn read_offset(spc700: &mut Spc700) -> i16 {
    (spc700.read_byte() as i8) as i16
}

fn branch_if(spc700: &mut Spc700, condition: bool) {
    let offset = read_offset(spc700);
    if condition {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(offset as u16);
    }
}

pub fn bra_rel(spc700: &mut Spc700) {
    let offset = read_offset(spc700);
    spc700.registers.pc = spc700.registers.pc.wrapping_add(offset as u16);
}

pub fn bpl_rel(spc700: &mut Spc700) {
    let cond = !spc700.registers.psw.negative();
    branch_if(spc700, cond);
}

pub fn bmi_rel(spc700: &mut Spc700) {
    let cond = spc700.registers.psw.negative();
    branch_if(spc700, cond);
}

pub fn bvc_rel(spc700: &mut Spc700) {
    let cond = !spc700.registers.psw.overflow();
    branch_if(spc700, cond);
}

pub fn bvs_rel(spc700: &mut Spc700) {
    let cond = spc700.registers.psw.overflow();
    branch_if(spc700, cond);
}

pub fn bcc_rel(spc700: &mut Spc700) {
    let cond = !spc700.registers.psw.carry();
    branch_if(spc700, cond);
}

pub fn bcs_rel(spc700: &mut Spc700) {
    let cond = spc700.registers.psw.carry();
    branch_if(spc700, cond);
}

pub fn bne_rel(spc700: &mut Spc700) {
    let cond = !spc700.registers.psw.zero();
    branch_if(spc700, cond);
}

pub fn beq_rel(spc700: &mut Spc700) {
    let cond = spc700.registers.psw.zero();
    branch_if(spc700, cond);
}

pub fn bbs(spc700: &mut Spc700, bit: u8) {
    let offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(offset));
    let rel = read_offset(spc700);
    if value & (1 << bit) != 0 {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(rel as u16);
    }
}

pub fn bbc(spc700: &mut Spc700, bit: u8) {
    let offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(offset));
    let rel = read_offset(spc700);
    if value & (1 << bit) == 0 {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(rel as u16);
    }
}

pub fn cbne_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let value = spc700.read(spc700.get_direct_page_address(offset));
    let rel = read_offset(spc700);
    if spc700.registers.a != value {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(rel as u16);
    }
}

pub fn cbne_dp_x(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let value = spc700.read(spc700.get_direct_page_address((offset + x) & 0xFF));
    let rel = read_offset(spc700);
    if spc700.registers.a != value {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(rel as u16);
    }
}

pub fn dbnz_y(spc700: &mut Spc700) {
    spc700.registers.y = spc700.registers.y.wrapping_sub(1);
    let rel = read_offset(spc700);
    if spc700.registers.y != 0 {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(rel as u16);
    }
}

pub fn dbnz_dp(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let address = spc700.get_direct_page_address(offset);
    let value = spc700.read(address).wrapping_sub(1);
    spc700.write(address, value);
    let rel = read_offset(spc700);
    if value != 0 {
        spc700.registers.pc = spc700.registers.pc.wrapping_add(rel as u16);
    }
}
