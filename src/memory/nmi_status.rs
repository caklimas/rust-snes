use bitfield::bitfield;

bitfield! {
    pub struct NmiStatus(u8);

    pub cpu_version, set_cpu_version: 3, 0;
    pub nmi_flag, set_nmi_flag: 7;
}

impl Default for NmiStatus {
    fn default() -> Self {
        Self(0x2)
    }
}
