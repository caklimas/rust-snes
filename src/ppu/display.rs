use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Display(u8);

    pub master_brightness, set_master_brightness: 3, 0;
    pub forced_blank, set_forced_blank: 7;
}
