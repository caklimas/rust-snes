use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Obsel(u8);

    pub name_base, _: 2, 0;
    pub name_select, _: 4, 3;
    pub object_size, _: 7, 6;
}

impl Obsel {
    pub fn get_object_size(&self, is_large: bool) -> u16 {
        match (self.object_size(), is_large) {
            (0, false) => 8,
            (1, false) => 8,
            (2, false) => 8,
            (3, false) => 16,
            (4, false) => 16,
            (5, false) => 32,
            (0, true) => 16,
            (1, true) => 32,
            (2, true) => 64,
            (3, true) => 32,
            (4, true) => 64,
            (5, true) => 64,
            (_, _) => unimplemented!(),
        }
    }
}
