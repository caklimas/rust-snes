use bitfield::bitfield;

bitfield! {

    #[derive(Clone, Copy, Default)]
    pub struct Control(u8);

    pub timer_enables, _: 2, 0;
    pub clear_cpuio_input_latch, _: 5, 4;
    pub ipl_rom_overlay, _: 7;
}
