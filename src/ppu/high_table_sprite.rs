use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct HighTableSprite(u8);

    pub x_position_bit_8, _: 0;
    pub size, _: 1;
}
