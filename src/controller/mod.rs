use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Controller(u16);

    pub r, set_r: 4;
    pub l, set_l: 5;
    pub x, set_x: 6;
    pub a, set_a: 7;
    pub right, set_right: 8;
    pub left, set_left: 9;
    pub down, set_down: 10;
    pub up, set_up: 11;
    pub start, set_start: 12;
    pub select, set_select: 13;
    pub y, set_y: 14;
    pub b, set_b: 15;

    pub low_byte, set_low_byte: 7, 0;
    pub high_byte, set_high_byte: 15, 8;
}
