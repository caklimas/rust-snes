use crate::apu::spc700::Spc700;

fn read_bit_address(spc700: &mut Spc700) -> (u32, u8) {
    let operand = spc700.read_word();
    let address = (operand & 0x1FFF) as u32;
    let bit = (operand >> 13) as u8;
    (address, bit)
}

pub fn set1(spc700: &mut Spc700, bit: u8) {
    let offset = spc700.read_byte() as u32;
    let address = spc700.get_direct_page_address(offset);
    let value = spc700.read(address);
    spc700.write(address, value | (1 << bit));
}

pub fn clr1(spc700: &mut Spc700, bit: u8) {
    let offset = spc700.read_byte() as u32;
    let address = spc700.get_direct_page_address(offset);
    let value = spc700.read(address);
    spc700.write(address, value & !(1 << bit));
}

pub fn not1(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = spc700.read(address);
    spc700.write(address, value ^ (1 << bit));
}

pub fn mov1_c_mem(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = spc700.read(address);
    spc700.registers.psw.set_carry((value >> bit) & 1 != 0);
}

pub fn mov1_mem_c(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = spc700.read(address);
    if spc700.registers.psw.carry() {
        spc700.write(address, value | (1 << bit));
    } else {
        spc700.write(address, value & !(1 << bit));
    }
}

pub fn or1(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = (spc700.read(address) >> bit) & 1 != 0;
    spc700
        .registers
        .psw
        .set_carry(spc700.registers.psw.carry() | value);
}

pub fn or1_not(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = (spc700.read(address) >> bit) & 1 != 0;
    spc700
        .registers
        .psw
        .set_carry(spc700.registers.psw.carry() | !value);
}

pub fn and1(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = (spc700.read(address) >> bit) & 1 != 0;
    spc700
        .registers
        .psw
        .set_carry(spc700.registers.psw.carry() & value);
}

pub fn and1_not(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = (spc700.read(address) >> bit) & 1 != 0;
    spc700
        .registers
        .psw
        .set_carry(spc700.registers.psw.carry() & !value);
}

pub fn eor1(spc700: &mut Spc700) {
    let (address, bit) = read_bit_address(spc700);
    let value = (spc700.read(address) >> bit) & 1 != 0;
    spc700
        .registers
        .psw
        .set_carry(spc700.registers.psw.carry() ^ value);
}

pub fn clrc(spc700: &mut Spc700) {
    spc700.registers.psw.set_carry(false);
}

pub fn setc(spc700: &mut Spc700) {
    spc700.registers.psw.set_carry(true);
}

pub fn notc(spc700: &mut Spc700) {
    let c = spc700.registers.psw.carry();
    spc700.registers.psw.set_carry(!c);
}

pub fn clrv(spc700: &mut Spc700) {
    spc700.registers.psw.set_overflow(false);
    spc700.registers.psw.set_half_carry(false);
}
