use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Wbglog(u8);

   pub bg1_combine_logic, _: 1, 0;
   pub bg2_combine_logic, _: 3, 2;
   pub bg3_combine_logic, _: 5, 4;
   pub bg4_combine_logic, _: 7, 6;
}
