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
| Mode 7 registers ($211A–$2120) | ❌ Not implemented | Writes logged but ignored |

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
| BG rendering — Mode 7 | ❌ Not implemented | Rotation/scaling matrix pipeline needed |
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
| SPC700 I/O ports ($2140–$217F) | ⚠️ Stubbed | IPL handshake echo; no actual SPC700 execution |
| SPC700 CPU struct | 🟡 In progress | `Spc700` struct with registers, 64KB RAM, IPL ROM, `step()` method; PC defaults to $FFC0 (reset vector) |
| SPC700 registers | ✅ Complete | `Registers` struct (A, X, Y, SP, PC, PSW); `ProcessorStatusWord` bitfield (N/V/P/B/H/I/Z/C) |
| SPC700 memory map | ✅ Complete | Read/write routing for RAM, IPL ROM overlay, I/O ports ($F0–$FF); CPUIO ($F4–$F7) stubbed (returns 0) pending Apu integration |
| SPC700 I/O ports ($F0–$FF) | ✅ Complete | `IoPorts` struct: TEST ($F0) no-op, CONTROL ($F1), DSPADDR/DSPDATA ($F2/$F3) with 128-byte stub DSP, AUX ($F8/$F9), timer dividers ($FA–$FC write-only), timer counters ($FD–$FF read-clear, 4-bit masked) |
| SPC700 CONTROL ($F1) | ✅ Complete | `Control` bitfield: `ipl_rom_overlay` bit 7, `timer_enables` bits 0–2, `clear_cpuio_input_latch` bits 4–5 |
| SPC700 CPUIO ($F4–$F7) | ✅ Complete | Bidirectional ports wired in `Apu`: main CPU side via $2140–$2143, SPC700 side via $00F4–$00F7; `cpu_to_spc`/`spc_to_cpu` arrays |
| SPC700 IPL ROM | ✅ Complete | 64-byte boot ROM embedded as `IPL_ROM` constant in `src/apu/constants.rs` |
| SPC700 instruction decoder | 🟡 In progress | `step()` + `read_byte()` (auto-advancing PC); 6 opcodes implemented: $5D MOV X,A / $BD MOV SP,X / $C6 MOV (X),A / $CD MOV X,#imm / $DD MOV A,Y / $E8 MOV A,#imm |
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

---

## Next Steps (Priority Order)

1. **SPC700 instruction decoder** — next: implement branch opcodes ($D0 BNE, $2F BRA, $10 BPL), compare opcodes ($78 CMP dp,#imm, $7E CMP Y,dp), then remaining IPL ROM opcodes ($8F MOV dp,#imm, $EB MOV Y,dp, $E4 MOV A,dp, $CB MOV dp,Y, $C4 MOV dp,A, $D7 MOV [dp]+Y,A, $FC INC Y, $AB INC dp, $1D DEC X, $BA MOVW YA,dp, $DA MOVW dp,YA, $1F JMP [abs+X]); wire SPC700 CPUIO ($F4–$F7) reads/writes through Apu port arrays; integrate step() into main emulation loop
2. **SPC700 timers** — T0–T2 tick logic needed by most sound drivers (storage already in place)
3. **Mode 7 rendering** — rotation/scaling matrix pipeline, EXTBG
4. **Offset-per-tile** — modes 2, 4, 6 use BG3 data for per-tile column/row offsets
5. **IRQ timer** — H/V count IRQ ($4207–$420A)
