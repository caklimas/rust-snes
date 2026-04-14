use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct M7sel(u8);

    pub horizontal_flip, _: 0;
    pub vertical_flip, _: 1;
    pub screen_over_mode, _: 7, 6;
}
