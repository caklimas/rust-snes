use crate::apu::{
    opcodes::{
        br::{bne_rel, bpl_rel, bra_rel},
        cmp::{cmp_dp_imm, cmp_y_dp},
        dec::dec_x,
        inc::{inc_dp, inc_y},
        jmp::jmp_abs_x,
        mov::{
            mov_a_dp, mov_a_imm, mov_a_y, mov_dp_a, mov_dp_imm, mov_dp_y, mov_dp_y_a, mov_ind_x_a,
            mov_sp_x, mov_x_a, mov_x_imm, mov_y_dp, movw_dp_ya, movw_ya_dp,
        },
    },
    spc700::Spc700,
};

pub mod br;
pub mod cmp;
pub mod dec;
pub mod inc;
pub mod jmp;
pub mod mov;

pub fn execute_opcode(spc700: &mut Spc700, opcode: u8) {
    match opcode {
        0x10 => bpl_rel(spc700),
        0x1D => dec_x(spc700),
        0x1F => jmp_abs_x(spc700),
        0x2F => bra_rel(spc700),
        0x5D => mov_x_a(spc700),
        0x78 => cmp_dp_imm(spc700),
        0x7E => cmp_y_dp(spc700),
        0x8F => mov_dp_imm(spc700),
        0xAB => inc_dp(spc700),
        0xBA => movw_ya_dp(spc700),
        0xBD => mov_sp_x(spc700),
        0xC4 => mov_dp_a(spc700),
        0xC6 => mov_ind_x_a(spc700),
        0xCB => mov_dp_y(spc700),
        0xCD => mov_x_imm(spc700),
        0xD0 => bne_rel(spc700),
        0xD7 => mov_dp_y_a(spc700),
        0xDA => movw_dp_ya(spc700),
        0xDD => mov_a_y(spc700),
        0xE4 => mov_a_dp(spc700),
        0xE8 => mov_a_imm(spc700),
        0xEB => mov_y_dp(spc700),
        0xFC => inc_y(spc700),
        _ => (),
    }
}
