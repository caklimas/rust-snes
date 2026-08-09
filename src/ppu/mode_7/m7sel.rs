use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct M7sel(u8);

    pub horizontal_flip, _: 0;
    pub vertical_flip, _: 1;
    pub screen_over_mode, _: 7, 6;
}

impl M7sel {
    pub fn get_screen_flips(&self, x: u16, y: u16) -> (u16, u16) {
        let sx = if self.horizontal_flip() { 255 - x } else { x };
        let sy = if self.vertical_flip() { 255 - y } else { y };

        (sx, sy)
    }
}
