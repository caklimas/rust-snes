# SNES Emulator — Implementation Status

This file tracks what has been implemented, what is stubbed, and what still needs work. Update it as features are completed.

---

## CPU (65C816)

| Component | Status |
|-----------|--------|
| All 256 opcodes (0x00–0xFF) | ✅ Complete |
| Emulation mode / native mode switching | ✅ Complete |
| NMI handling | ✅ Complete |
| IRQ handling | ✅ Complete |
| WAI / STP | ✅ Complete |

---

## Memory Bus

| Component | Status | Notes |
|-----------|--------|-------|
| WRAM ($7E0000–$7FFFFF) | ✅ Complete | |
| WRAM mirror ($00–$3F, $80–$BF, offsets $0000–$1FFF) | ✅ Complete | |
| WRAM access ports ($2180–$2183) | ✅ Complete | Auto-increment, 17-bit wrap |
| Bank normalization ($80–$BF → $00–$3F) | ✅ Complete | |
| LoROM mapping | ✅ Complete | |
| HiROM mapping | ✅ Complete | $80–$BF mirror fixed |
| ExHiROM mapping | ⚠️ Stubbed | |
| Cartridge SRAM | ⚠️ Stubbed | |

---

## PPU

| Component | Status | Notes |
|-----------|--------|-------|
| VRAM ($2115–$2119) | ✅ Complete | |
| CGRAM ($2121–$2122) | ✅ Complete | |
| OAM ($2102–$2104) | ✅ Complete | |
| INIDISP ($2100) | ✅ Complete | forced_blank, master_brightness |
| OBSEL ($2101) | ✅ Complete | name_base, name_select, object_size (3-bit) |
| BGMODE ($2105) | ✅ Complete | |
| BG1SC–BG4SC ($2107–$210A) | ✅ Complete | |
| BG12NBA / BG34NBA ($210B–$210C) | ✅ Complete | |
| BGxHOFS / BGxVOFS ($210D–$2114) | ✅ Complete | M7 latch formula |
| TM / TS ($212C–$212D) | ✅ Complete | Main/sub screen designation |
| SETINI ($2133) | ✅ Complete | |
| 16x16 tile support | ✅ Complete | Per-layer via BGMODE bits 4–7, quadrant flip handling |
| BG rendering — Mode 0 | ✅ Complete | 2bpp, per-BG palette bands |
| BG rendering — Mode 1 | ✅ Complete | BG1/BG2 4bpp, BG3 2bpp |
| BG rendering — Mode 2 | ✅ Complete | 4bpp BG1/BG2 (via BppSettings) |
| BG rendering — Mode 3 | ✅ Complete | 8bpp BG1, 4bpp BG2 |
| BG rendering — Modes 4–7 | ❌ Not implemented | |
| Sprite (OAM) rendering | ✅ Complete | 4bpp, priority, x/y flip, multi-tile |
| Priority compositing | ⚠️ Partial | Modes 0, 1 (with BG3 boost), 2, 3 via `PriorityResolver`; Modes 4–7 not yet |
| Windowing ($2123–$212B) | ❌ Not implemented | |
| Color math / sub-screen blending ($2130–$2132) | ❌ Not implemented | |
| Mode 7 | ❌ Not implemented | |

---

## DMA / HDMA

| Component | Status | Notes |
|-----------|--------|-------|
| DMA ($420B, $4300–$437F) | ✅ Complete | Modes 0/1/2, both directions, fixed transfer |
| HDMA ($420C) | ✅ Complete | Direct mode; repeat mode always-transfer (refine later) |

---

## APU

| Component | Status | Notes |
|-----------|--------|-------|
| SPC700 I/O ports ($2140–$217F) | ⚠️ Stubbed | IPL handshake echo; no actual SPC700 execution |
| SPC700 CPU | ❌ Not implemented | |
| DSP / audio output | ❌ Not implemented | |

---

## I/O

| Component | Status | Notes |
|-----------|--------|-------|
| NMITIMEN ($4200) | ✅ Complete | NMI enable, auto-joypad enable |
| NMI status ($4210) | ✅ Complete | Clears on read |
| HVBJOY ($4212) | ✅ Complete | vblank/hblank flags |
| MDMAEN ($420B) | ✅ Complete | |
| HDMAEN ($420C) | ✅ Complete | |
| Joypad auto-read ($4218–$421F) | ✅ Complete | Controller 1 via InputOutput struct |
| Keyboard input (winit) | ✅ Complete | Arrows=d-pad, Z=B, X=A, A=Y, S=X, Q=L, W=R, Enter=Start, RShift=Select |
| Joypad serial ($4016–$4017) | ❌ Not implemented | |
| IRQ timer ($4207–$420A) | ❌ Not implemented | |
| MEMSEL ($420D) | ✅ Complete | FastROM enable for WS2 banks via `MemorySelect` bitfield |
| CPU I/O range ($4200–$5FFF) remainder | ⚠️ Stubbed | Returns 0 |
| Joypad I/O range ($4000–$41FF) | ⚠️ Stubbed | Returns 0 |

---

## Display / Windowing (Host)

| Component | Status | Notes |
|-----------|--------|-------|
| Window creation (winit 0.30) | ✅ Complete | |
| Framebuffer rendering (softbuffer 0.4) | ✅ Complete | BGR555 → u32, nearest-neighbour scale |
| Frame pacing (vblank-driven) | ✅ Complete | `frame_complete()` gates redraws |

---

## Next Steps (Priority Order)

1. **Multi-screen tilemap layout** — 64-wide/tall tilemaps are two 32x32 screens at +0x400 offsets, not linear; fix entry_address calculation in `bg_sample`
3. **Master brightness** — apply INIDISP bits 3–0 to rendered pixels (fade-in/fade-out)
4. **Color math / blending** — $2130–$2132
5. **Windowing** — $2123–$212B
6. **Additional BG modes** — Modes 4, 5, 6, 7 (including priority compositing)
7. **SPC700** — full audio emulation
