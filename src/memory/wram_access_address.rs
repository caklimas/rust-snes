use bitfield::bitfield;

bitfield! {
    #[derive(Default)]
    pub struct WramAccessAddress(u32);

    pub wmaddl, set_wmaddl: 7, 0;
    pub wmaddm, set_wmaddm: 15, 8;
    pub wmaddh, set_wmaddh: 16;
}
