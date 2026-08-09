# SNES Emulator — Implementation Status

This file tracks what has been implemented, what is stubbed, and what still needs work. Update it as features are completed.

---

## CPU (65C816)

| Component | Status |
|-----------|--------|
| All 256 opcodes (0x00–0xFF) | ✅ Complete |
| Emulation mode / native mode switching | ✅ Complete |
| NMI handling | ✅ Complete |
| IRQ handling | ✅ Complete | Native ($FFEE/EF) + emulation ($FFFE/FF) vectors; shared `enter_interrupt` helper with NMI; dispatched from `SuperNintendo::step` when `irq_pending && !I` |
| WAI / STP | ✅ Complete |

---

## Memory Bus

| Component | Status | Notes |
|-----------|--------|-------|
| WRAM ($7E0000–$7FFFFF) | ✅ Complete | |
| WRAM mirror ($00–$3F, $80–$BF, offsets $0000–$1FFF) | ✅ Complete | |
| WRAM access ports ($2180–$2183) | ✅ Complete | Auto-increment, 17-bit wrap |
| System Area mirroring ($00–$3F / $80–$BF, offsets $0000–$5FFF) | ✅ Complete | Internal WRAM and I/O addresses normalize to bank $00; cartridge-dependent $6000+ offsets retain their bank |
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
| SLHV ($2137) | ✅ Complete | Latches current_x and current_scanline into ophct_latch/opvct_latch; returns 0 (open bus) |
| OPHCT / OPVCT ($213C–$213D) | ✅ Complete | Two-read flipflop: 1st read = low 8 bits, 2nd read = high bit; flipflops reset on $213F read |
| STAT77 ($213E) | 🟡 Partial | Returns 0x01 (PPU1 version 1); OBJ overflow flags not implemented |
| STAT78 ($213F) | ✅ Complete | `Stat78` bitfield (`src/ppu/stat78.rs`); defaults to 0x03 (PPU2 v3, NTSC); read clears latch_flag + both OPHCT/OPVCT flipflops; interlace_frame_counter toggles per frame when SETINI bit 0 set, forced 0 otherwise |

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
| BG rendering — Mode 7 | 🟡 In progress | Affine transform + tile/pixel lookup + brightness + OBJ compositing implemented; F-Zero white screen fixed (HDMA non-repeat); rendering shows garbage — likely affine math or coordinate issues (see `docs/bugs/mode7-fzero-white-screen.md`); color math not yet applied |
| Offset-per-tile (Modes 2, 4, 6) | ❌ Not implemented | BG3 used as per-tile offset source |
| 16x16 tile support | ✅ Complete | Per-layer via BGMODE bits 4–7, quadrant flip |
| Multi-screen tilemap layout | ✅ Complete | 64-wide/tall via SC register bits |
| Sprite (OAM) rendering | ✅ Complete | 4bpp, priority, x/y flip (tile + sub-tile), multi-tile, Y screen-relative |
| Mosaic | ✅ Complete | Per-BG enable, grid-aligned snap in BgSampleParams |
| Priority compositing — Mode 0 | ✅ Complete | All 4 BG layers + OBJ |
| Priority compositing — Mode 1 | ✅ Complete | BG3 priority boost |
| Priority compositing — Modes 2–5 | ✅ Complete | BG1/BG2 + OBJ |
| Priority compositing — Mode 6 | ✅ Complete | BG1 only + OBJ |
| Priority compositing — Mode 7 | ✅ Complete (non-EXTBG) | OBJ pri 3 > BG1 > OBJ pri 2/1/0 > backdrop; EXTBG not yet |
| Windowing | ✅ Complete | W12SEL/W34SEL/WOBJSEL, WH0–WH3, WBGLOG/WOBJLOG, TMW/TSW |
| Color math — fixed color | ✅ Complete | CGWSEL, CGADSUB, COLDATA; add/subtract, half-math |
| Color math — sub-screen blending | ✅ Complete | Sub screen rendered independently; suppress_div2 |
| Color math — window gating | ✅ Complete | WOBJSEL instance 2 + WOBJLOG math_combine_logic |
| Color math — OBJ palette split | ✅ Complete | Palettes 0-3 (CGRAM <192) exempt from color math; only palettes 4-7 respect CGADSUB bit 4 |
| Color math — force main screen black | ✅ Complete | CGWSEL bits 7-6; evaluates math window; forces color to Rgb(0) before blend; suppresses half-math |
| Master brightness | ✅ Complete | `channel * (brightness + 1) / 16` |
| VRAM write guard | ✅ Complete | Blocks writes during active rendering unless forced_blank |

---

## DMA / HDMA

| Component | Status | Notes |
|-----------|--------|-------|
| DMA ($420B, $4300–$437F) | ✅ Complete | Modes 0/1/2/3/4, both directions, fixed transfer |
| HDMA ($420C) | ✅ Complete | Direct mode, transfer modes 0-4; non-repeat mode implemented (do_transfer = bit 7 of decremented counter) |

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
| SPC700 instruction decoder | ✅ Complete | All 256 opcodes implemented; integrated into main loop with clock accumulator (768 SPC clocks per 1364 main clocks) |
| SPC700 execution integration | ✅ Complete | `spc_clocks: i32` accumulator on `SuperNintendo`; SPC700 steps proportionally alongside main CPU |
| SPC700 timers (T0–T2) | ✅ Complete | Tick logic in `Timer::tick()`; T0/T1 every 128 SPC clocks (8 kHz), T2 every 16 (64 kHz); enable gated by CONTROL bits 0–2; ticked from `Spc700::step()` |
| DSP / audio output | ❌ Not implemented | |

---

## I/O

| Component | Status | Notes |
|-----------|--------|-------|
| NMITIMEN ($4200) | ✅ Complete | NMI enable (bit 7), H/V-IRQ mode (bits 5-4, `HVIrqMode` enum), auto-joypad (bit 0). Write of 0 to bits 5-4 acknowledges pending IRQ |
| NMI status ($4210) | ✅ Complete | Clears on read |
| HVBJOY ($4212) | 🟡 Partial | vblank set/cleared at scanline 225/0; hblank approximated via master-clock threshold (`HBLANK_START_MASTER_CLOCKS = 1096`, ~17% of scanline) — unblocks F-Zero spin loops; bit 0 (auto-joypad busy) not implemented |
| HTIMEL/HTIMEH ($4207/$4208) | ✅ Complete | 9-bit H-count trigger |
| VTIMEL/VTIMEH ($4209/$420A) | ✅ Complete | 9-bit V-count trigger |
| TIMEUP ($4211) | ✅ Complete | Read returns H/V-IRQ pending flag in bit 7; read-to-clear. Hardware edge case (read during active 4–8 master cycles doesn't clear) not implemented |
| MDMAEN ($420B) | ✅ Complete | |
| HDMAEN ($420C) | ✅ Complete | |
| Hardware multiply ($4202–$4203, $4216–$4217) | ✅ Complete | 8x8→16 unsigned, result instant on $4203 write |
| Hardware divide ($4204–$4206, $4214–$4217) | ✅ Complete | 16÷8 unsigned, quotient+remainder instant on $4206 write; div-by-zero→$FFFF |
| Joypad auto-read ($4218–$421F) | ✅ Complete | Controller 1 via InputOutput struct |
| Keyboard input (winit) | ✅ Complete | Arrows=d-pad, Z=B, X=A, A=Y, S=X, Q=L, W=R, Enter=Start, RShift=Select |
| Joypad serial ($4016–$4017) | ❌ Not implemented | |
| MEMSEL ($420D) | ✅ Complete | FastROM enable for WS2 banks |
| CPU I/O range ($4200–$5FFF) remainder | ⚠️ Stubbed | Returns 0 |
| Joypad I/O range ($4000–$41FF) | ⚠️ Stubbed | Returns 0 |

---

## Display (Host)

| Component | Status | Notes |
|-----------|--------|-------|
| Window creation (winit 0.30) | ✅ Complete | |
| Framebuffer rendering (softbuffer 0.4) | ✅ Complete | BGR555 → u32, nearest-neighbour scale. 5-bit→8-bit conversion uses canonical `(x<<3)\|(x>>2)` so channel extremes reach 0/255 |
| Frame pacing (vblank-driven) | ✅ Complete | `frame_complete()` gates redraws |
| Pause (P key) | ✅ Complete | Toggles emulation; completes current frame before stopping |
| Debug dump (D key, paused) | ✅ Complete | Writes CPU, SPC700, PPU state + NMI handler bytes to `docs/bugs/debug_dump.txt` |
| Frame buffer dump (F key, paused) | ✅ Complete | Writes PNG image to `docs/bugs/frame_<timestamp>.png` via the `png` crate |

---

## Known Bugs

### F-Zero title screen — Mode 7 gradient defect on scanlines 47–83
- **Symptom**: Pixel diff against reference shows a smooth descending gradient of channel deltas: 124 at row 47 (horizon), decreasing by ~8 per scanline down to 9 at row 83. ~3,400 differing pixels total. Visually subtle but cleanly localized to the Mode 7 region.
- **What's confirmed NOT the cause**: HDMA off-by-one (tested by shifting all HDMA timing by 1 scanline — made things much worse, 8,302 differing pixels, broke the COLDATA gradient and rows 80-159). IRQ timing (now fires at correct scanline; verified V=47/86 IRQs land correctly). 5→8-bit color conversion (canonical formula in both live + dump paths).
- **Diagnostic fingerprint**: smooth-gradient delta scaling with perspective compression. Largest at horizon (where many VRAM coords map to few screen pixels), shrinks toward foreground. Each step is ~8 in 8-bit space = 1 in 5-bit CGRAM space.
- **Where to look next**: see `docs/bugs/fzero-mode7-gradient.md`. Most likely: subtle bug in Mode 7 affine math or per-scanline M7B accumulation. NOT a register-write timing issue.
- **Investigation tooling in place**: `Ppu::scanline_trace` now captures per-scanline Mode 7 register state (m7a/b/c/d/x/y, m7hofs/vofs, m7sel) when `bg_mode==7`. Surfaces in debug_dump.txt via P+D.

## Recently Resolved

### LttP Triforce intro — missing Triforce graphic
- **Symptom**: The copyright text rendered, but the CPU-rasterized Triforce polygons were missing.
- **Root cause**: System Area I/O mirroring was incomplete. LttP reads the Mode 7 multiplication result through `$09:2135`, but the bus only routed `$00:2135` to the PPU. The bad multiplication results collapsed every projected vertex to one point, so the game generated blank OBJ tile data.
- **Fix**: Normalize internal System Area addresses in banks `$00–$3F` and `$80–$BF` to bank `$00`, while preserving cartridge-dependent offsets at `$6000` and above.

---

## Next Steps (Priority Order)

1. **LttP overworld garbled tiles — IRQ timing bug** — per-scanline trace shows NO register changes across 224 scanlines. LttP's IRQ (VMatch at scanline 144) should fire mid-frame to change BG3 scroll/TM/etc. between HUD and game area. IRQ total fire count is >0 but "last frame" shows 0 entries. BG3 has priority boost (BGMODE bit 3), so unclipped BG3 HUD tiles appear over the entire screen. Investigate: why does the IRQ not fire during the rendered frame? Check VMatch evaluation in `super_nintendo/mod.rs` step loop.
2. **LttP text scrolling** — text doesn't scroll up; new text garbles into old. Game rewrites BG3 tilemap directly (NOT via VOFS). Likely a VRAM tilemap update issue — old tiles not being cleared. May be related to DMA or VRAM address handling.
3. **Mode 7 color math missing** — Mode 7 renderer skips the entire color math pipeline (found via bsnes comparison). Needs to apply same CGWSEL/CGADSUB/sub-screen logic as modes 0-6.
4. **SMW — Yoshi's tongue doesn't fully extend** — sprite positioning issue, not yet investigated.
5. **Mode 7 gradient defect (F-Zero scanlines 47–83)** — narrowed to Mode 7 affine math or per-scanline M7B handling. See `docs/bugs/fzero-mode7-gradient.md`.
6. **Offset-per-tile** — modes 2, 4, 6 use BG3 data for per-tile column/row offsets
7. **DSP / audio output** — needed for actual sound
