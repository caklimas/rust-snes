use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct PackedAttributes(u8);

    pub name_table, _: 0;
    pub palette, _: 3, 1;
    pub priority, _: 5, 4;
    pub x_flip, _: 6;
    pub y_flip, _: 7;
}
