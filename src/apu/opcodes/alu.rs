use crate::apu::spc700::Spc700;

#[derive(Clone, Copy)]
pub enum AluOp {
    Or,
    And,
    Eor,
    Cmp,
    Adc,
    Sbc,
}

fn apply_alu(spc700: &mut Spc700, op: AluOp, a: u8, b: u8) -> u8 {
    match op {
        AluOp::Or => {
            let result = a | b;
            spc700.set_n(result);
            spc700.set_z(result);
            result
        }
        AluOp::And => {
            let result = a & b;
            spc700.set_n(result);
            spc700.set_z(result);
            result
        }
        AluOp::Eor => {
            let result = a ^ b;
            spc700.set_n(result);
            spc700.set_z(result);
            result
        }
        AluOp::Cmp => {
            let result = a.wrapping_sub(b);
            spc700.set_n(result);
            spc700.set_z(result);
            spc700.set_c(a, b);
            a
        }
        AluOp::Adc => {
            let carry = spc700.registers.psw.carry() as u16;
            let sum = (a as u16) + (b as u16) + carry;
            let result = sum as u8;
            spc700.set_n(result);
            spc700.set_z(result);
            spc700.registers.psw.set_carry(sum > 0xFF);
            spc700
                .registers
                .psw
                .set_half_carry((a & 0x0F) + (b & 0x0F) + (carry as u8) > 0x0F);
            spc700
                .registers
                .psw
                .set_overflow(((a ^ b) & 0x80) == 0 && ((a ^ result) & 0x80) != 0);
            result
        }
        AluOp::Sbc => {
            let borrow = !spc700.registers.psw.carry() as u16;
            let diff = (a as u16).wrapping_sub((b as u16) + borrow);
            let result = diff as u8;
            spc700.set_n(result);
            spc700.set_z(result);
            spc700.registers.psw.set_carry(diff <= 0xFF);
            spc700.registers.psw.set_half_carry(
                (a & 0x0F).wrapping_sub(b & 0x0F).wrapping_sub(borrow as u8) <= 0x0F,
            );
            spc700
                .registers
                .psw
                .set_overflow((((a ^ b) & 0x80) != 0) && (((a ^ result) & 0x80) != 0));
            result
        }
    }
}

// A, #imm
pub fn alu_a_imm(spc700: &mut Spc700, op: AluOp) {
    let imm = spc700.read_byte();
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, imm);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, (X)
pub fn alu_a_ind_x(spc700: &mut Spc700, op: AluOp) {
    let address = spc700.get_direct_page_address(spc700.registers.x as u32);
    let b = spc700.read(address);
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, dp
pub fn alu_a_dp(spc700: &mut Spc700, op: AluOp) {
    let offset = spc700.read_byte() as u32;
    let b = spc700.read(spc700.get_direct_page_address(offset));
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, dp+X
pub fn alu_a_dp_x(spc700: &mut Spc700, op: AluOp) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let address = spc700.get_direct_page_address((offset + x) & 0xFF);
    let b = spc700.read(address);
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, abs
pub fn alu_a_abs(spc700: &mut Spc700, op: AluOp) {
    let address = spc700.read_word() as u32;
    let b = spc700.read(address);
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, abs+X
pub fn alu_a_abs_x(spc700: &mut Spc700, op: AluOp) {
    let address = spc700.read_word() as u32;
    let b = spc700.read(address.wrapping_add(spc700.registers.x as u32));
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, abs+Y
pub fn alu_a_abs_y(spc700: &mut Spc700, op: AluOp) {
    let address = spc700.read_word() as u32;
    let b = spc700.read(address.wrapping_add(spc700.registers.y as u32));
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, [dp]+Y
pub fn alu_a_ind_dp_y(spc700: &mut Spc700, op: AluOp) {
    let offset = spc700.read_byte() as u32;
    let pointer = spc700.read_word_direct(spc700.get_direct_page_address(offset)) as u32;
    let b = spc700.read(pointer.wrapping_add(spc700.registers.y as u32));
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// A, [dp+X]
pub fn alu_a_ind_dp_x(spc700: &mut Spc700, op: AluOp) {
    let offset = spc700.read_byte() as u32;
    let x = spc700.registers.x as u32;
    let pointer =
        spc700.read_word_direct(spc700.get_direct_page_address((offset + x) & 0xFF)) as u32;
    let b = spc700.read(pointer);
    let a = spc700.registers.a;
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.registers.a = result;
    }
}

// dp, dp
pub fn alu_dp_dp(spc700: &mut Spc700, op: AluOp) {
    let src_offset = spc700.read_byte() as u32;
    let dst_offset = spc700.read_byte() as u32;
    let b = spc700.read(spc700.get_direct_page_address(src_offset));
    let dst_address = spc700.get_direct_page_address(dst_offset);
    let a = spc700.read(dst_address);
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.write(dst_address, result);
    }
}

// dp, #imm
pub fn alu_dp_imm(spc700: &mut Spc700, op: AluOp) {
    let imm = spc700.read_byte();
    let offset = spc700.read_byte() as u32;
    let dp_address = spc700.get_direct_page_address(offset);
    let a = spc700.read(dp_address);
    let result = apply_alu(spc700, op, a, imm);
    if !matches!(op, AluOp::Cmp) {
        spc700.write(dp_address, result);
    }
}

// (X), (Y)
pub fn alu_ind_x_ind_y(spc700: &mut Spc700, op: AluOp) {
    let y_addr = spc700.get_direct_page_address(spc700.registers.y as u32);
    let b = spc700.read(y_addr);
    let x_addr = spc700.get_direct_page_address(spc700.registers.x as u32);
    let a = spc700.read(x_addr);
    let result = apply_alu(spc700, op, a, b);
    if !matches!(op, AluOp::Cmp) {
        spc700.write(x_addr, result);
    }
}
