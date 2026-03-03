use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct ScreenSetting(u8);

    pub screen_interlace, _: 0;
    pub obj_interlace, _: 1;
    pub overscan, _: 2;
    pub pseudo_hires, _: 3;
    pub enable_bg2_mode_7, _: 6;
    pub external_sync, _: 7;
}
