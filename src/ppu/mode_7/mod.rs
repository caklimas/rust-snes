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

    pub fn get_vram_coords(&self, screen_x: u16, screen_y: u16) -> (i32, i32) {
        let m7a = self.affine_matrix.m7a as i32;
        let m7b = self.affine_matrix.m7b as i32;
        let m7c = self.affine_matrix.m7c as i32;
        let m7d = self.affine_matrix.m7d as i32;
        let m7x = self.rotation_scaling.m7x as i32;
        let m7y = self.rotation_scaling.m7y as i32;

        let org_x = Self::clip13(self.scroll_offset.m7hofs as i32 - m7x);
        let org_y = Self::clip13(self.scroll_offset.m7vofs as i32 - m7y);

        let sx = screen_x as i32;
        let sy = screen_y as i32;

        let vram_x = ((m7a * org_x) & !0x3F)
            + ((m7b * org_y) & !0x3F)
            + (m7x << 8)
            + ((m7b * sy) & !0x3F)
            + (m7a * sx);

        let vram_y = ((m7c * org_x) & !0x3F)
            + ((m7d * org_y) & !0x3F)
            + (m7y << 8)
            + ((m7d * sy) & !0x3F)
            + (m7c * sx);

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

    /// Per fullsnes `ppu.md:297-298`, Mode 7's "ORG" computation clips the
    /// `M7HOFS - M7X` (or `M7VOFS - M7Y`) subtraction with this unusual bit op:
    /// mask out bits 10-12, then if the masked result is negative, set bits 10-12
    /// back. This is NOT the same as a clean 13-bit sign-extend — values like
    /// `0x0800` (2048) collapse to `0`, not to `-2048`.
    fn clip13(input: i32) -> i32 {
        let input16 = input as i16 as i32;
        let masked = input16 & !0x1C00;
        if masked < 0 { masked | 0x1C00 } else { masked }
    }
}
