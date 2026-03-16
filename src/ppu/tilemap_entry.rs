use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct TilemapEntry(u16);

    pub tile_number, _: 9, 0;
    pub palette_number, _: 12, 10;
    pub tile_priority, _: 13;
    pub x_flip, _: 14;
    pub y_flip, _: 15;
}
