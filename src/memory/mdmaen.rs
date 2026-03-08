use bitfield::bitfield;

bitfield! {
    pub struct Mdmaen(u8);

    pub channel_1, _: 0;
    pub channel_2, _: 1;
    pub channel_3, _: 2;
    pub channel_4, _: 3;
    pub channel_5, _: 4;
    pub channel_6, _: 5;
    pub channel_7, _: 6;
    pub channel_8, _: 7;
}
