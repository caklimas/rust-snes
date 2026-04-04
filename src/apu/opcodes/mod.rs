use crate::apu::{
    opcodes::mov::{mov_a_imm, mov_a_y, mov_ind_x_a, mov_sp_x, mov_x_a, mov_x_imm},
    spc700::Spc700,
};

pub mod mov;

pub fn execute_opcode(spc700: &mut Spc700, opcode: u8) {
    match opcode {
        0x5D => mov_x_a(spc700),
        0xBD => mov_sp_x(spc700),
        0xC6 => mov_ind_x_a(spc700),
        0xCD => mov_x_imm(spc700),
        0xDD => mov_a_y(spc700),
        0xE8 => mov_a_imm(spc700),
        _ => unimplemented!(),
    }
}
