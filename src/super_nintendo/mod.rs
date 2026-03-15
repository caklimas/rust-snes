use crate::{
    cpu::Cpu,
    memory::{
        addresses::{RESET_VECTOR_HI, RESET_VECTOR_LO},
        bus::Bus,
    },
};

const MASTER_CLOCKS_PER_SCANLINE: u32 = 1364;
const DRAM_REFRESH_MASTER_CLOCKS: u32 = 40;

pub struct SuperNintendo {
    pub bus: Bus,
    cpu: Cpu,
    current_scanline: u16,
    master_clocks: u32,
    frame_complete: bool,
}

impl SuperNintendo {
    pub fn new(data: Vec<u8>) -> Self {
        let mut bus = Bus::new(data);
        let mut cpu = Cpu::default();

        let lo = bus.read(RESET_VECTOR_LO);
        let hi = bus.read(RESET_VECTOR_HI);
        cpu.registers.pc = u16::from_le_bytes([lo, hi]);

        Self {
            bus,
            cpu,
            current_scanline: 0,
            master_clocks: 0,
            frame_complete: false,
        }
    }

    pub fn step(&mut self) {
        // Approximate master clock cost: cycles × speed of region CPU is executing from
        let pc_address = ((self.cpu.registers.pb as u32) << 16) | (self.cpu.registers.pc as u32);
        let clocks_per_cycle = self.bus.master_clocks_for_address(pc_address);
        let cpu_cycles = self.cpu.step(&mut self.bus) as u32;
        self.master_clocks += cpu_cycles * clocks_per_cycle;

        if self.master_clocks >= MASTER_CLOCKS_PER_SCANLINE {
            self.master_clocks -= MASTER_CLOCKS_PER_SCANLINE;
            // Account for DRAM refresh: charge 40 master clocks per scanline
            self.master_clocks += DRAM_REFRESH_MASTER_CLOCKS;

            if self.current_scanline < 224 {
                self.bus.run_hdma_scanline();
                self.bus.ppu.render_scanline(self.current_scanline);
            }

            self.current_scanline = (self.current_scanline + 1) % 262;

            if self.current_scanline == 225 {
                self.bus.nmi_status.set_nmi_flag(true);
                self.bus.hvbjoy.set_vblank(true);

                if self.bus.interrupt_enable.nmi_enable() {
                    self.cpu.nmi(&mut self.bus);
                }
            }

            if self.current_scanline == 0 {
                self.bus.init_hdma();
                self.bus.hvbjoy.set_vblank(false);
                self.frame_complete = true;
            }
        }
    }

    pub fn frame_buffer(&self) -> &[u16] {
        self.bus.frame_buffer()
    }

    pub fn frame_complete(&mut self) -> bool {
        let frame_complete = self.frame_complete;
        if frame_complete {
            self.frame_complete = false;
        }

        frame_complete
    }
}
