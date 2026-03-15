use bitfield::bitfield;

bitfield! {

    #[derive(Clone, Copy, Default)]
    pub struct BgMode(u8);

    pub bg_mode, set_bg_mode: 2, 0;
    pub bg_priority_boost, set_bg_priority_boost: 3;
    pub tile_size_1, set_tile_size_1: 4;
    pub tile_size_2, set_tile_size_2: 5;
    pub tile_size_3, set_tile_size_3: 6;
    pub tile_size_4, set_tile_size_4: 7;
}
