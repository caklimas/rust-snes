use bitfield::bitfield;

bitfield! {

    #[derive(Default)]
    pub struct WindowMaskSettings(u8);

   pub instance_1_window_1, _: 1, 0;
   pub instance_1_window_2, _: 3, 2;
   pub instance_2_window_1, _: 5, 4;
   pub instance_2_window_2, _: 7, 6;
}
