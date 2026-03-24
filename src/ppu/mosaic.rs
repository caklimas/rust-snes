use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Mosaic(u8);

    pub bg1_enable, _: 0;
    pub bg2_enable, _: 1;
    pub bg3_enable, _: 2;
    pub bg4_enable, _: 3;
    pub mosaic_size, _: 7, 4;
}
