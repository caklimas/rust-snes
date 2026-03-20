use bitfield::bitfield;

bitfield! {

    #[derive(Clone, Copy, Default)]
    pub struct Rgb(u16);

    pub red, set_red: 4, 0;
    pub green, set_green: 9, 5;
    pub blue, set_blue: 14, 10;
}
