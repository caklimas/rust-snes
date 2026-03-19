use bitfield::bitfield;

use crate::ppu::window_condition::WindowCondition;

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct Cgwsel(u8);

    pub direct_color_mode, _: 0;
    pub sub_screen_enable, _: 1;
    pub color_math_enable, _: 5, 4;
    pub force_main_screen_black, _: 7, 6;
}

impl Cgwsel {
    pub fn get_color_math_enable(&self) -> WindowCondition {
        match self.color_math_enable() {
            0 => WindowCondition::Always,
            1 => WindowCondition::MathWindow,
            2 => WindowCondition::NotMathWin,
            3 => WindowCondition::Never,
            _ => unimplemented!(),
        }
    }

    pub fn get_force_main_screen_black(&self) -> WindowCondition {
        match self.force_main_screen_black() {
            0 => WindowCondition::Never,
            1 => WindowCondition::NotMathWin,
            2 => WindowCondition::MathWindow,
            3 => WindowCondition::Always,
            _ => unimplemented!(),
        }
    }
}
