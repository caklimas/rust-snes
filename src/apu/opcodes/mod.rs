use crate::apu::{
    opcodes::{
        br::{bne_rel, bpl_rel, bra_rel},
        cmp::{cmp_dp_imm, cmp_y_dp},
        mov::{mov_a_imm, mov_a_y, mov_ind_x_a, mov_sp_x, mov_x_a, mov_x_imm},
    },
    spc700::Spc700,
};

pub mod br;
pub mod cmp;
pub mod mov;

pub fn execute_opcode(spc700: &mut Spc700, opcode: u8) {
    match opcode {
        0x10 => bpl_rel(spc700),
        0x2F => bra_rel(spc700),
        0x5D => mov_x_a(spc700),
        0x78 => cmp_dp_imm(spc700),
        0x7E => cmp_y_dp(spc700),
        0xBD => mov_sp_x(spc700),
        0xC6 => mov_ind_x_a(spc700),
        0xCD => mov_x_imm(spc700),
        0xD0 => bne_rel(spc700),
        0xDD => mov_a_y(spc700),
        0xE8 => mov_a_imm(spc700),
        _ => unimplemented!(),
    }
}

pub fn get_direct_page_address(spc700: &Spc700, offset: u32) -> u32 {
    ((spc700.registers.psw.direct_page() as u32) * 0x100) | offset
}
