use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct PackedAttributes(u8);

    pub name_table, _: 0;
    pub y_flip, _: 1;
    pub x_flip, _: 2;
    pub palette, _: 5, 3;
    pub priority, _: 7, 6;
}
