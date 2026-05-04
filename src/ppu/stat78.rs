use bitfield::bitfield;

bitfield! {

    #[derive(Clone, Copy)]
    pub struct Stat78(u8);

    pub ppu2_version, _: 3, 0;
    pub frame_rate, _: 4;
    pub ppu2_open_bus, _: 5;
    pub latch_flag, set_latch_flag: 6;
    pub interlace_frame_counter, set_interlace_frame_counter: 7;
}

impl Default for Stat78 {
    fn default() -> Self {
        Self(0x03)
    }
}
