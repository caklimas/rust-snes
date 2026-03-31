use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct Cgadsub(u8);

    pub bg1, _: 0;
    pub bg2, _: 1;
    pub bg3, _: 2;
    pub bg4, _: 3;
    pub obj, _: 4;
    pub backdrop, _: 5;
    pub half_math, _: 6;
    pub add_or_subtract, _: 7;
}
