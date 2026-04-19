use std::{cell::RefCell, rc::Rc};

use crate::{
    apu::{Apu, spc700::Spc700},
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
    pub cpu: Cpu,
    pub spc700: Spc700,
    current_scanline: u16,
    frame_complete: bool,
    master_clocks: u32,
    spc_clocks: i32,
}

impl SuperNintendo {
    pub fn new(data: Vec<u8>) -> Self {
        let apu = Rc::new(RefCell::new(Apu::default()));
        let mut bus = Bus::new(apu.clone(), data);
        let mut cpu = Cpu::default();

        let lo = bus.read(RESET_VECTOR_LO);
        let hi = bus.read(RESET_VECTOR_HI);
        cpu.registers.pc = u16::from_le_bytes([lo, hi]);

        Self {
            bus,
            cpu,
            current_scanline: 0,
            frame_complete: false,
            master_clocks: 0,
            spc700: Spc700::new(apu.clone()),
            spc_clocks: 0,
        }
    }

    pub fn step(&mut self) {
        // Approximate master clock cost: cycles × speed of region CPU is executing from
        let pc_address = ((self.cpu.registers.pb as u32) << 16) | (self.cpu.registers.pc as u32);
        let clocks_per_cycle = self.bus.master_clocks_for_address(pc_address);
        let cpu_cycles = self.cpu.step(&mut self.bus) as u32;
        let cpu_master_clocks = cpu_cycles * clocks_per_cycle;
        self.master_clocks += cpu_master_clocks;
        self.spc_clocks += (cpu_master_clocks * 768 / 1364) as i32;

        while self.spc_clocks > 0 {
            self.spc700.step();
            self.spc_clocks -= 1; // refine with actual cycle counts later
        }

        if self.master_clocks >= MASTER_CLOCKS_PER_SCANLINE {
            self.master_clocks -= MASTER_CLOCKS_PER_SCANLINE;
            // Account for DRAM refresh: charge 40 master clocks per scanline
            self.master_clocks += DRAM_REFRESH_MASTER_CLOCKS;

            if self.current_scanline >= 1 && self.current_scanline <= 224 {
                self.bus.run_hdma_scanline();
                self.bus.ppu.render_scanline(self.current_scanline);
            }

            self.current_scanline = (self.current_scanline + 1) % 262;
            self.bus.ppu.current_scanline = self.current_scanline;

            if self.current_scanline == 225 {
                self.bus.ppu.oam.reset_address();
                self.bus.ppu.vram.rendering_active = false;
                self.bus.nmi_status.set_nmi_flag(true);
                self.bus.hvbjoy.set_vblank(true);

                if self.bus.interrupt_enable.nmi_enable() {
                    self.cpu.nmi(&mut self.bus);
                }
            }

            if self.current_scanline == 0 {
                self.bus.ppu.vram.rendering_active = !self.bus.ppu.display.forced_blank();
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

    pub fn debug_info(&mut self) -> String {
        // Read NMI vector and first 32 bytes of the handler
        let nmi_lo = self.bus.read(0x00FFEA) as u16;
        let nmi_hi = self.bus.read(0x00FFEB) as u16;
        let nmi_addr = (nmi_hi << 8) | nmi_lo;

        let mut handler_bytes = String::new();
        for i in 0u16..32 {
            let byte = self.bus.read(nmi_addr.wrapping_add(i) as u32);
            handler_bytes.push_str(&format!("{:02X} ", byte));
        }

        let mode_7_trace = self.bus.ppu.scanline_trace.join("\n");

        // Dump BG1 tilemap entries (row 29, 32 tiles from each screen = 64 tiles total)
        // BG1 base = 0x7800 word. For ms=1 (64x32): screen 0 at base, screen 1 at base+0x400.
        // Row 29, local_y=29 -> offset = 29*32 = 928 = 0x3A0.
        let mut bg1_row29 = String::new();
        bg1_row29.push_str("\nBG1 tilemap row 29 (left screen @ 0x7BA0..0x7BBF, right @ 0x7FA0..0x7FBF):\n  LEFT : ");
        for i in 0..32 {
            let word = self.bus.ppu.vram.read_word(0x7800 + 0x3A0 + i);
            bg1_row29.push_str(&format!("{:04X} ", word));
        }
        bg1_row29.push_str("\n  RIGHT: ");
        for i in 0..32 {
            let word = self.bus.ppu.vram.read_word(0x7800 + 0x400 + 0x3A0 + i);
            bg1_row29.push_str(&format!("{:04X} ", word));
        }
        // Also dump row 4 (logo row) for comparison — should be the logo tiles
        bg1_row29.push_str("\nBG1 tilemap row 4 (should be logo area):\n  LEFT : ");
        for i in 0..32 {
            let word = self.bus.ppu.vram.read_word(0x7800 + 4 * 32 + i);
            bg1_row29.push_str(&format!("{:04X} ", word));
        }
        bg1_row29.push_str("\n  RIGHT: ");
        for i in 0..32 {
            let word = self.bus.ppu.vram.read_word(0x7800 + 0x400 + 4 * 32 + i);
            bg1_row29.push_str(&format!("{:04X} ", word));
        }
        // Tile 384's char data (word 0x7800..0x780F for 4bpp tile)
        bg1_row29.push_str("\nTile 384 char data (word 0x7800..0x780F — collides with BG1 tilemap!):\n  ");
        for i in 0..16 {
            let word = self.bus.ppu.vram.read_word(0x7800 + i);
            bg1_row29.push_str(&format!("{:04X} ", word));
        }
        // BG1 tilemap rows 0..3 (above row 4) — what's at the very top
        bg1_row29.push_str("\nBG1 tilemap rows 0..3 (left screen only):");
        for row in 0..4 {
            bg1_row29.push_str(&format!("\n  row{}: ", row));
            for i in 0..32 {
                let word = self.bus.ppu.vram.read_word(0x7800 + row * 32 + i);
                bg1_row29.push_str(&format!("{:04X} ", word));
            }
        }
        // BG2 tilemap rows 11..17 (logo area) — what should contain the logo
        bg1_row29.push_str("\nBG2 tilemap rows 11..17 (logo area, left screen):");
        for row in 11..18 {
            bg1_row29.push_str(&format!("\n  row{:2}: ", row));
            for i in 0..32 {
                let word = self.bus.ppu.vram.read_word(0x7000 + row * 32 + i);
                bg1_row29.push_str(&format!("{:04X} ", word));
            }
        }
        // BG2 tilemap rows 17..22 (road area start)
        bg1_row29.push_str("\nBG2 tilemap rows 17..22 (road area, left screen):");
        for row in 17..22 {
            bg1_row29.push_str(&format!("\n  row{:2}: ", row));
            for i in 0..32 {
                let word = self.bus.ppu.vram.read_word(0x7000 + row * 32 + i);
                bg1_row29.push_str(&format!("{:04X} ", word));
            }
        }
        // Char data for key tiles used on this screen (4bpp, 16 words each)
        bg1_row29.push_str("\nChar data for sky tiles 1,5,10 (garbled in render) and logo tile 38 (correct):");
        for tile in &[1u16, 5, 10, 38, 70] {
            bg1_row29.push_str(&format!("\n  tile{:3} @ word 0x{:04X}: ", tile, 0x6000 + tile * 16));
            for i in 0..16 {
                let word = self.bus.ppu.vram.read_word(0x6000 + tile * 16 + i);
                bg1_row29.push_str(&format!("{:04X} ", word));
            }
        }
        // CGRAM palette 7 (indices 112..127) vs palette 0 (indices 0..15)
        bg1_row29.push_str("\nCGRAM palette 0 (indices 0..15):\n  ");
        for i in 0..16u8 {
            let color = self.bus.ppu.cgram.read_color(i as u16);
            bg1_row29.push_str(&format!("{:04X} ", color));
        }
        bg1_row29.push_str("\nCGRAM palette 7 (indices 112..127):\n  ");
        for i in 112..128u16 {
            let color = self.bus.ppu.cgram.read_color(i);
            bg1_row29.push_str(&format!("{:04X} ", color));
        }

        format!(
            "{:#?}\n{:#?}\n{:#?}\nNMI vector: ${:04X}\nNMI handler bytes: {}\n\n\
             --- Per-scanline trace (captured last frame) ---\n{}\n\
             --- BG1 VRAM tilemap sample ---{}",
            self.cpu, self.spc700, self.bus.ppu, nmi_addr, handler_bytes, mode_7_trace, bg1_row29
        )
    }
}
