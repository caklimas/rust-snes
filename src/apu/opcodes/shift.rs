use crate::apu::spc700::Spc700;

#[derive(Clone, Copy)]
pub enum ShiftOp {
    Asl,
    Rol,
    Lsr,
    Ror,
}

fn apply_shift(spc700: &mut Spc700, op: ShiftOp, value: u8) -> u8 {
    let carry = spc700.registers.psw.carry() as u8;
    let result = match op {
        ShiftOp::Asl => {
            spc700.registers.psw.set_carry(value & 0x80 != 0);
            value << 1
        }
        ShiftOp::Rol => {
            spc700.registers.psw.set_carry(value & 0x80 != 0);
            (value << 1) | carry
        }
        ShiftOp::Lsr => {
            spc700.registers.psw.set_carry(value & 0x01 != 0);
            value >> 1
        }
        ShiftOp::Ror => {
            spc700.registers.psw.set_carry(value & 0x01 != 0);
            (value >> 1) | (carry << 7)
        }
    };
    spc700.set_n(result);
    spc700.set_z(result);
    result
}

pub fn shift_a(spc700: &mut Spc700, op: ShiftOp) {
    let value = spc700.registers.a;
    spc700.registers.a = apply_shift(spc700, op, value);
}

pub fn shift_dp(spc700: &mut Spc700, op: ShiftOp) {
    let offset = spc700.read_byte() as u32;
    let address = spc700.get_direct_page_address(offset);
    let value = spc700.read(address);
    let result = apply_shift(spc700, op, value);
    spc700.write(address, result);
}

pub fn shift_dp_x(spc700: &mut Spc700, op: ShiftOp) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let address = spc700.get_direct_page_address((offset + x) & 0xFF);
    let value = spc700.read(address);
    let result = apply_shift(spc700, op, value);
    spc700.write(address, result);
}

pub fn shift_abs(spc700: &mut Spc700, op: ShiftOp) {
    let address = spc700.read_word() as u32;
    let value = spc700.read(address);
    let result = apply_shift(spc700, op, value);
    spc700.write(address, result);
}
