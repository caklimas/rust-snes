use bitfield::bitfield;

bitfield! {
    #[derive(Default)]
    pub struct InterruptEnable(u8);

    pub joypad_read_enable, _: 0;
    pub irq_enable, _: 4;
    pub nmi_enable, _: 7;
}
