use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct Wobjlog(u8);

   pub obj_combine_logic, _: 1, 0;
   pub math_combine_logic, _: 3, 2;
}
