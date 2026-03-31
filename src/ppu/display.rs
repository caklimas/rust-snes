use bitfield::bitfield;

bitfield! {

    pub struct Display(u8);

    pub master_brightness, set_master_brightness: 3, 0;
    pub forced_blank, set_forced_blank: 7;
}

impl Default for Display {
    fn default() -> Self {
        Self(0x80)
    }
}
