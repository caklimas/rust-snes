use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct Coldata(u8);

    pub intensity, _: 4, 0;
    pub apply_to_red, _: 5;
    pub apply_to_green, _: 6;
    pub apply_to_blue, _: 7;
}
