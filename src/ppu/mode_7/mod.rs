use crate::{
    memory::addresses::{BG1HOFS, BG1VOFS, M7A, M7B, M7C, M7D, M7SEL, M7X, M7Y, MPYH, MPYL, MPYM},
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
    pub multiply_result: i32,
    pub rotation_scaling: RotationScaling,
    pub scroll_offset: ScrollOffset,
}

impl Mode7 {
    pub fn read(&mut self, address: u32) -> u8 {
        match address {
            MPYL => self.multiply_result as u8,
            MPYM => (self.multiply_result >> 8) as u8,
            MPYH => (self.multiply_result >> 16) as u8,
            _ => unimplemented!(),
        }
    }

    pub fn write(&mut self, address: u32, value: u8) {
        match address {
            M7SEL => self.m7sel.0 = value,
            M7A => self.affine_matrix.m7a = self.get_affine_value(value),
            M7B => {
                self.affine_matrix.m7b = self.get_affine_value(value);
                self.multiply_result = (self.affine_matrix.m7a as i32) * (value as i8 as i32);
            }
            M7C => self.affine_matrix.m7c = self.get_affine_value(value),
            M7D => self.affine_matrix.m7d = self.get_affine_value(value),
            M7X => self.rotation_scaling.m7x = self.get_rotation_scaling_value(value),
            M7Y => self.rotation_scaling.m7y = self.get_rotation_scaling_value(value),
            BG1HOFS => self.scroll_offset.m7hofs = self.get_rotation_scaling_value(value),
            BG1VOFS => self.scroll_offset.m7vofs = self.get_rotation_scaling_value(value),
            _ => unimplemented!(),
        }
    }

    pub fn get_origin_relative_coords(&self, sx: u16, sy: u16) -> (i32, i32) {
        let org_x = sx as i32
            + Self::clip13(self.scroll_offset.m7hofs as i32 - self.rotation_scaling.m7x as i32);
        let org_y = sy as i32
            + Self::clip13(self.scroll_offset.m7vofs as i32 - self.rotation_scaling.m7y as i32);

        (org_x, org_y)
    }

    pub fn get_affine_transform(&self, org_x: i32, org_y: i32) -> (i32, i32) {
        let vram_x = self.affine_matrix.m7a as i32 * org_x
            + self.affine_matrix.m7b as i32 * org_y
            + ((self.rotation_scaling.m7x as i32) << 8);

        let vram_y = self.affine_matrix.m7c as i32 * org_x
            + self.affine_matrix.m7d as i32 * org_y
            + ((self.rotation_scaling.m7y as i32) << 8);

        (vram_x, vram_y)
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

    fn clip13(input: i32) -> i32 {
        (input << 19) >> 19
    }
}
