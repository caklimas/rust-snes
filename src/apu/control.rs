use bitfield::bitfield;

bitfield! {

    #[derive(Clone, Copy)]
    pub struct Control(u8);

    pub timer_enables, _: 2, 0;
    pub clear_cpuio_input_latch, _: 5, 4;
    pub ipl_rom_overlay, _: 7;
}

impl Control {
    pub fn is_timer_enabled(&self, timer_index: usize) -> bool {
        self.timer_enables() & (1 << timer_index) != 0
    }
}

impl Default for Control {
    fn default() -> Self {
        Self(0x80)
    }
}
