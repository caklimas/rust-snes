use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Vmain(u8);

    pub increment_amount, set_increment_amount: 1, 0;
    pub address_translation, set_address_translation: 3, 2;
    pub increment_timing, set_increment_timing: 7;
}
