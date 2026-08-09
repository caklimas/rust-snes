use crate::apu::spc700::Spc700;

fn get_ya(spc700: &Spc700) -> u16 {
    ((spc700.registers.y as u16) << 8) | (spc700.registers.a as u16)
}

fn set_ya(spc700: &mut Spc700, value: u16) {
    spc700.registers.a = value as u8;
    spc700.registers.y = (value >> 8) as u8;
}

fn set_nz_word(spc700: &mut Spc700, value: u16) {
    spc700.registers.psw.set_negative(value & 0x8000 != 0);
    spc700.registers.psw.set_zero(value == 0);
}

pub fn addw(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let lo = spc700.read(dp_address) as u16;
    let hi = spc700.read(dp_address.wrapping_add(1)) as u16;
    let operand = (hi << 8) | lo;
    let ya = get_ya(spc700);
    let sum = (ya as u32) + (operand as u32);
    let result = sum as u16;
    set_ya(spc700, result);
    set_nz_word(spc700, result);
    spc700.registers.psw.set_carry(sum > 0xFFFF);
    spc700
        .registers
        .psw
        .set_half_carry((ya & 0x0FFF) + (operand & 0x0FFF) > 0x0FFF);
    spc700
        .registers
        .psw
        .set_overflow(((ya ^ operand) & 0x8000) == 0 && ((ya ^ result) & 0x8000) != 0);
}

pub fn subw(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let lo = spc700.read(dp_address) as u16;
    let hi = spc700.read(dp_address.wrapping_add(1)) as u16;
    let operand = (hi << 8) | lo;
    let ya = get_ya(spc700);
    let diff = (ya as u32).wrapping_sub(operand as u32);
    let result = diff as u16;
    set_ya(spc700, result);
    set_nz_word(spc700, result);
    spc700.registers.psw.set_carry(diff <= 0xFFFF);
    spc700
        .registers
        .psw
        .set_half_carry((ya & 0x0FFF) >= (operand & 0x0FFF));
    spc700
        .registers
        .psw
        .set_overflow(((ya ^ operand) & 0x8000 != 0) && ((ya ^ result) & 0x8000 != 0));
}

pub fn cmpw(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let lo = spc700.read(dp_address) as u16;
    let hi = spc700.read(dp_address.wrapping_add(1)) as u16;
    let operand = (hi << 8) | lo;
    let ya = get_ya(spc700);
    let result = ya.wrapping_sub(operand);
    set_nz_word(spc700, result);
    spc700.registers.psw.set_carry(ya >= operand);
}

pub fn incw(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let lo = spc700.read(dp_address) as u16;
    let hi = spc700.read(dp_address.wrapping_add(1)) as u16;
    let result = ((hi << 8) | lo).wrapping_add(1);
    spc700.write(dp_address, result as u8);
    spc700.write(dp_address.wrapping_add(1), (result >> 8) as u8);
    set_nz_word(spc700, result);
}

pub fn decw(spc700: &mut Spc700) {
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let lo = spc700.read(dp_address) as u16;
    let hi = spc700.read(dp_address.wrapping_add(1)) as u16;
    let result = ((hi << 8) | lo).wrapping_sub(1);
    spc700.write(dp_address, result as u8);
    spc700.write(dp_address.wrapping_add(1), (result >> 8) as u8);
    set_nz_word(spc700, result);
}

pub fn mul_ya(spc700: &mut Spc700) {
    let result = (spc700.registers.y as u16) * (spc700.registers.a as u16);
    set_ya(spc700, result);
    spc700.set_n(spc700.registers.y);
    spc700.set_z(spc700.registers.y);
}

pub fn div_ya_x(spc700: &mut Spc700) {
    let x = spc700.registers.x as u16;
    let y = spc700.registers.y as u16;
    let ya = get_ya(spc700);

    spc700.registers.psw.set_overflow(y >= x);
    spc700
        .registers
        .psw
        .set_half_carry((y & 0x0F) >= (x & 0x0F));

    if x == 0 {
        spc700.registers.a = 0xFF;
        spc700.registers.y = ya as u8;
    } else if y < x * 2 {
        spc700.registers.a = (ya / x) as u8;
        spc700.registers.y = (ya % x) as u8;
    } else {
        let denom = 256u16.wrapping_sub(x);
        if denom == 0 {
            spc700.registers.a = 0xFF;
            spc700.registers.y = x as u8;
        } else {
            let adjusted = ya.wrapping_sub(x.wrapping_mul(512));
            spc700.registers.a = (255u16.wrapping_sub(adjusted / denom)) as u8;
            spc700.registers.y = (x.wrapping_add(adjusted % denom)) as u8;
        }
    }

    spc700.set_n(spc700.registers.a);
    spc700.set_z(spc700.registers.a);
}
