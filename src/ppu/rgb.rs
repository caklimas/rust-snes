use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Rgb(u16);

    pub red, _: 4, 0;
    pub green, _: 9, 5;
    pub blue, _: 14, 10;
}
