use bitfield::bitfield;

bitfield! {
    #[derive(Clone, Copy, Default)]
    pub struct MemorySelect(u8);

    pub access_speed, _: 0;
}
