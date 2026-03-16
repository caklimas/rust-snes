use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct DmaParameter(u8);

    pub transfer_mode, _: 2, 0;
    pub fixed_transfer, _: 3;
    pub indirect_hdma, _: 6;
    pub transfer_direction, _: 7;
}
