use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct WindowLayerDisable(u8);

    pub bg1_disable, _: 0;
    pub bg2_disable, _: 1;
    pub bg3_disable, _: 2;
    pub bg4_disable, _: 3;
    pub obj_disable, _: 4;
}
