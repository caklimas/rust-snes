use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct Hvbjoy(u8);

    pub auto_joypad, _: 0;
    pub hblank, set_hblank: 6;
    pub vblank, set_vblank: 7;
}
