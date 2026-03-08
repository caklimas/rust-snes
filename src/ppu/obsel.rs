use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Obsel(u8);

    pub name_base, _: 2, 0;
    pub name_select, _: 4, 3;
    pub object_size, _: 7, 6;
}
