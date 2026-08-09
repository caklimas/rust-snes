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

### Registers

| Register | Status | Notes |
|----------|--------|-------|
| INIDISP ($2100) | ✅ Complete | forced_blank, master_brightness; defaults to 0x80 (forced blank on) |
| OBSEL ($2101) | ✅ Complete | name_base, name_select, object_size (3-bit) |
| BGMODE ($2105) | ✅ Complete | bg_mode, bg_priority_boost, per-BG tile_size |
| MOSAIC ($2106) | ✅ Complete | Per-BG enable bits 0–3, mosaic_size bits 7–4; snaps x/y in BgSampleParams |
| BG1SC–BG4SC ($2107–$210A) | ✅ Complete | |
| BG12NBA / BG34NBA ($210B–$210C) | ✅ Complete | |
| BGxHOFS / BGxVOFS ($210D–$2114) | ✅ Complete | Shared bg_old latch |
| VMAIN ($2115) | ✅ Complete | Increment mode and amount |
| VMADDL/VMADDH ($2116–$2117) | ✅ Complete | |
| VMDATAL/VMDATAH ($2118–$2119) | ✅ Complete | Write guard checks rendering_active + forced_blank |
| CGADD/CGDATA ($2121–$2122) | ✅ Complete | |
| W12SEL/W34SEL/WOBJSEL ($2123–$2125) | ✅ Complete | |
| WH0–WH3 ($2126–$2129) | ✅ Complete | |
| WBGLOG/WOBJLOG ($212A–$212B) | ✅ Complete | |
| TM / TS ($212C–$212D) | ✅ Complete | Main/sub screen designation |
| TMW / TSW ($212E–$212F) | ✅ Complete | |
| CGWSEL ($2130) | ✅ Complete | |
| CGADSUB ($2131) | ✅ Complete | |
| COLDATA ($2132) | ✅ Complete | |
| SETINI ($2133) | ✅ Complete | |
| Mode 7 registers ($211A–$2120) | ✅ Complete | M7SEL, M7A–D, M7X/M7Y, M7HOFS/M7VOFS; double-write via m7_old latch; 13-bit sign-extend for center/scroll |

### Rendering

| Component | Status | Notes |
|-----------|--------|-------|
| Scanline timing | ✅ Complete | V counter 1–224 visible; V used for scroll math, V-1 for frame buffer index |
| BG rendering — Mode 0 | ✅ Complete | 2bpp all 4 layers, per-BG palette bands |
| BG rendering — Mode 1 | ✅ Complete | BG1/BG2 4bpp, BG3 2bpp |
| BG rendering — Mode 2 | ✅ Complete | BG1/BG2 4bpp |
| BG rendering — Mode 3 | ✅ Complete | BG1 8bpp, BG2 4bpp |
| BG rendering — Mode 4 | ✅ Complete | BG1 8bpp, BG2 2bpp |
| BG rendering — Mode 5 | ✅ Complete | BG1 4bpp, BG2 2bpp |
| BG rendering — Mode 6 | ✅ Complete | BG1 4bpp only |
| BG rendering — Mode 7 | 🟡 In progress | Affine transform + tile/pixel lookup implemented; not yet tested (no game has triggered Mode 7); missing brightness, color math, OBJ compositing in Mode 7 path |
| Offset-per-tile (Modes 2, 4, 6) | ❌ Not implemented | BG3 used as per-tile offset source |
| 16x16 tile support | ✅ Complete | Per-layer via BGMODE bits 4–7, quadrant flip |
| Multi-screen tilemap layout | ✅ Complete | 64-wide/tall via SC register bits |
| Sprite (OAM) rendering | ✅ Complete | 4bpp, priority, x/y flip (tile + sub-tile), multi-tile, Y screen-relative |
| Mosaic | ✅ Complete | Per-BG enable, grid-aligned snap in BgSampleParams |
| Priority compositing — Mode 0 | ✅ Complete | All 4 BG layers + OBJ |
| Priority compositing — Mode 1 | ✅ Complete | BG3 priority boost |
| Priority compositing — Modes 2–5 | ✅ Complete | BG1/BG2 + OBJ |
| Priority compositing — Mode 6 | ✅ Complete | BG1 only + OBJ |
| Priority compositing — Mode 7 | ❌ Not implemented | Per-pixel priority for EXTBG |
| Windowing | ✅ Complete | W12SEL/W34SEL/WOBJSEL, WH0–WH3, WBGLOG/WOBJLOG, TMW/TSW |
| Color math — fixed color | ✅ Complete | CGWSEL, CGADSUB, COLDATA; add/subtract, half-math |
| Color math — sub-screen blending | ✅ Complete | Sub screen rendered independently; suppress_div2 |
| Color math — window gating | ✅ Complete | WOBJSEL instance 2 + WOBJLOG math_combine_logic |
| Master brightness | ✅ Complete | `channel * (brightness + 1) / 16` |
| VRAM write guard | ✅ Complete | Blocks writes during active rendering unless forced_blank |

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
| SPC700 I/O ports ($2140–$217F) | ✅ Complete | Bidirectional via `Rc<RefCell<Apu>>` shared between `Bus` and `Spc700` |
| SPC700 CPU struct | ✅ Complete | `Spc700` struct with registers, 64KB RAM, IPL ROM, `step()` method; PC defaults to $FFC0 |
| SPC700 registers | ✅ Complete | `Registers` struct (A, X, Y, SP, PC, PSW); `ProcessorStatusWord` bitfield (N/V/P/B/H/I/Z/C) |
| SPC700 memory map | ✅ Complete | Read/write routing for RAM, IPL ROM overlay, I/O ports ($F0–$FF); CPUIO ($F4–$F7) via shared `Apu` |
| SPC700 I/O ports ($F0–$FF) | ✅ Complete | `IoPorts` struct: TEST ($F0) no-op, CONTROL ($F1 — defaults 0x80, overlay on), DSPADDR/DSPDATA ($F2/$F3) with 128-byte stub DSP, AUX ($F8/$F9), timer dividers ($FA–$FC write-only), timer counters ($FD–$FF read-clear, 4-bit masked) |
| SPC700 CONTROL ($F1) | ✅ Complete | `Control` bitfield: `ipl_rom_overlay` bit 7 (defaults on), `timer_enables` bits 0–2, `clear_cpuio_input_latch` bits 4–5 |
| SPC700 CPUIO ($F4–$F7) | ✅ Complete | `Rc<RefCell<Apu>>` shared between Bus and Spc700; main CPU side via $2140–$2143, SPC700 side via $00F4–$00F7 |
| SPC700 IPL ROM | ✅ Complete | 64-byte boot ROM embedded as `IPL_ROM` constant; IPL handshake verified working (LttP boots) |
| SPC700 instruction decoder | 🟡 In progress | 23 IPL ROM opcodes implemented; unimplemented opcodes log and skip; integrated into main loop with clock accumulator (768 SPC clocks per 1364 main clocks) |
| SPC700 execution integration | ✅ Complete | `spc_clocks: i32` accumulator on `SuperNintendo`; SPC700 steps proportionally alongside main CPU |
| SPC700 timers (T0–T2) | ❌ Not implemented | Divider/counter storage in place, no tick logic yet |
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
| Hardware multiply ($4202–$4203, $4216–$4217) | ✅ Complete | 8x8→16 unsigned, result instant on $4203 write |
| Hardware divide ($4204–$4206, $4214–$4217) | ✅ Complete | 16÷8 unsigned, quotient+remainder instant on $4206 write; div-by-zero→$FFFF |
| Joypad auto-read ($4218–$421F) | ✅ Complete | Controller 1 via InputOutput struct |
| Keyboard input (winit) | ✅ Complete | Arrows=d-pad, Z=B, X=A, A=Y, S=X, Q=L, W=R, Enter=Start, RShift=Select |
| Joypad serial ($4016–$4017) | ❌ Not implemented | |
| IRQ timer ($4207–$420A) | ❌ Not implemented | |
| MEMSEL ($420D) | ✅ Complete | FastROM enable for WS2 banks |
| CPU I/O range ($4200–$5FFF) remainder | ⚠️ Stubbed | Returns 0 |
| Joypad I/O range ($4000–$41FF) | ⚠️ Stubbed | Returns 0 |

---

## Display (Host)

| Component | Status | Notes |
|-----------|--------|-------|
| Window creation (winit 0.30) | ✅ Complete | |
| Framebuffer rendering (softbuffer 0.4) | ✅ Complete | BGR555 → u32, nearest-neighbour scale |
| Frame pacing (vblank-driven) | ✅ Complete | `frame_complete()` gates redraws |
| Pause (P key) | ✅ Complete | Toggles emulation; completes current frame before stopping |
| Debug dump (D key, paused) | ✅ Complete | Writes CPU, SPC700, PPU state + NMI handler bytes to `docs/bugs/debug_dump.txt` |
| Frame buffer dump (F key, paused) | ✅ Complete | Writes PPM image to `docs/bugs/frame_<timestamp>.ppm` |

---

## Known Bugs

### LttP Triforce intro — missing Triforce graphic
- **Symptom**: The "1991, 1992" copyright text renders at the bottom, but the Triforce above it is missing. The rest of the intro works fine.
- **Root cause (suspected)**: The Triforce is NOT a static asset or Mode 7 — it is **CPU-rasterized polygons written directly into VRAM each frame** (15 polygons at 60fps, software-rendered by the CPU). The game's CPU calculates triangle fill during VBlank and writes the pixel data into VRAM as tile data. The PPU renders it as normal BG tiles.
- **Likely culprit**: VRAM write guard timing. `write_data_lo`/`write_data_hi` gate writes on `!rendering_active || forced_blank()`. If `rendering_active` isn't being cleared at the right time during VBlank, CPU-driven VRAM writes are silently dropped.
- **How to investigate**: Add a trace to `write_data_lo`/`write_data_hi` during the Triforce screen to see if writes are being blocked. Check that `rendering_active` transitions align with VBlank timing.

---

## Next Steps (Priority Order)

1. **Mode 7 rendering — finish and test** — registers and affine transform implemented; needs brightness/color math/OBJ compositing in the Mode 7 render path; needs a game or ROM that actually triggers Mode 7 to verify (LttP map screen, F-Zero, or Super Mario Kart)
2. **SPC700 opcodes** — implement remaining opcodes as games hit them (currently logs unimplemented opcodes and skips)
3. **SPC700 timers** — T0–T2 tick logic needed by most sound drivers (storage already in place)
4. **Offset-per-tile** — modes 2, 4, 6 use BG3 data for per-tile column/row offsets
5. **IRQ timer** — H/V count IRQ ($4207–$420A)
