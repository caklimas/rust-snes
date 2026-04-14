use crate::{
    memory::addresses::{BG1HOFS, BG1VOFS, M7A, M7B, M7C, M7D, M7SEL, M7X, M7Y},
    ppu::mode_7::{
        affine_matrix::AffineMatrix, m7sel::M7sel, rotation_scaling::RotationScaling,
        scroll_offset::ScrollOffset,
    },
};

pub mod affine_matrix;
pub mod m7sel;
pub mod rotation_scaling;
pub mod scroll_offset;

#[derive(Default)]
pub struct Mode7 {
    pub affine_matrix: AffineMatrix,
    pub m7sel: M7sel,
    pub m7_old: u8,
    pub rotation_scaling: RotationScaling,
    pub scroll_offset: ScrollOffset,
}

impl Mode7 {
    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            M7SEL => self.m7sel.0 = value,
            M7A => self.affine_matrix.m7a = self.get_affine_value(value),
            M7B => self.affine_matrix.m7b = self.get_affine_value(value),
            M7C => self.affine_matrix.m7c = self.get_affine_value(value),
            M7D => self.affine_matrix.m7d = self.get_affine_value(value),
            M7X => self.rotation_scaling.m7x = self.get_rotation_scaling_value(value),
            M7Y => self.rotation_scaling.m7y = self.get_rotation_scaling_value(value),
            BG1HOFS => self.scroll_offset.m7hofs = self.get_rotation_scaling_value(value),
            BG1VOFS => self.scroll_offset.m7vofs = self.get_rotation_scaling_value(value),
            _ => unimplemented!(),
        }
    }

    fn get_affine_value(&mut self, value: u8) -> i16 {
        let updated_value = ((value as i16) << 8) | (self.m7_old as i16);
        self.m7_old = value;

        updated_value
    }

    fn get_rotation_scaling_value(&mut self, value: u8) -> i16 {
        let updated_value = self.get_affine_value(value);

        (updated_value << 3) >> 3
    }
}
