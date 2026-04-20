use bitfield::bitfield;

bitfield! {
    #[derive(Default)]
    pub struct InterruptEnable(u8);

    pub joypad_read_enable, _: 0;
    pub h_v_irq_enable, _: 5, 4;
    pub nmi_enable, _: 7;
}

impl InterruptEnable {
    pub fn h_v_irq_mode(&self) -> HVIrqMode {
        match self.h_v_irq_enable() {
            0 => HVIrqMode::Disabled,
            1 => HVIrqMode::HMatch,
            2 => HVIrqMode::VMatch,
            3 => HVIrqMode::HAndVMatch,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum HVIrqMode {
    Disabled,
    HMatch,
    VMatch,
    HAndVMatch,
}
