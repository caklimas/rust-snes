# F-Zero Mode 7 Gradient Defect (rows 47–83)

## Status

Active. Earlier hypothesis ("HDMA off-by-one") tested and disproven this session.

## Symptom

F-Zero title screen renders the correct overall layout. Pixel diff against `F-Zero-title.png` shows a smooth descending gradient of channel deltas in the Mode 7 region:

```
row 47: max_delta=124 (horizon, where Mode 7 starts)
row 48: max_delta=124
row 49: max_delta=116
row 50: max_delta=108
...
row 75: max_delta=17
row 79: max_delta=9
row 83: max_delta=9
```

Each step is approximately 8 in 8-bit channel space, which equals 1 in 5-bit CGRAM color space. ~3,400 differing pixels total. Below row 83 the diff is clean.

## Diagnostic fingerprint

The gradient *scales with perspective compression*: largest at the horizon (V=47), shrinks toward the foreground (V=83). At V=47, many VRAM source coordinates map to few screen pixels (extreme compression), so any per-pixel error in the affine math is magnified. At V=83, each screen pixel maps to fewer VRAM coords, so errors are visually smaller.

This rules out "uniform shift" bugs (HDMA off-by-one, scanline alignment, etc.) — those would produce a constant delta, not a smooth gradient.

## What's been ruled out this session

| Hypothesis | How tested | Result |
|---|---|---|
| HDMA timing off-by-one (entry transition lands one scanline late) | Modified `init_hdma()` to call `run_hdma_scanline()` at end, shifting all HDMA timing | **Made it dramatically worse** (3,413 → 8,302 differing pixels). All HDMA-driven values shifted by 1 scanline, breaking COLDATA gradient and ALL rows 80-159. Reverted. |
| IRQ off-by-one | Moved `current_scanline += 1` to before IRQ-match check in `super_nintendo/mod.rs` | **Real improvement** (3,707 → 3,413, fully cleaned the row 105-118 band that was the V=86 IRQ effect). Kept. |
| Frame dump color conversion | Found `app/mod.rs:226-228` using `<<3` instead of `(<<3)|(>>2)`. Fixed | Eliminated 5-7-channel uniform delta in upper non-Mode-7 rows. Kept. |
| Mode 7 affine formula match to spec | Pending changes apply spec-literal formula from `ppu.md:297-302` (per-partial-product `& ~0x3F`, then `+ M7A*SCREEN.X`) and the `clip13` AND-NOT-1C00h truncation | Build clean, formulas match spec. Did not eliminate the gradient — bug is elsewhere. |

## Trace evidence

After all session fixes (IRQ + STAT78 + dump color), at scanlines 47-50:

```
y= 47 bgmode=0x07 bg1_hofs= 224 bg1_vofs= 36   ← Mode 7 enabled, but BG1HOFS still Mode 1 value
  m7  m7a=30464 m7b=-30209 m7c=30464 m7d=30464 m7x=1136 m7y=616 m7hofs=224 m7vofs=36
y= 48 bgmode=0x07 bg1_hofs=1008 bg1_vofs=440
  m7  m7a=    0 m7b= -1280 m7c=  1280 m7d=    0 m7x=1136 m7y=616 m7hofs=1008 m7vofs=440
y= 49 bgmode=0x07 ... m7b=-1185
y= 50 bgmode=0x07 ... m7b=-1103
```

Key observations:
1. BGMODE switch from Mode 1 (0x09) to Mode 7 (0x07) lands correctly at scanline 47 (set by IRQ-47, which now fires at the correct scanline).
2. M7A-D HDMA writes (channels 4-7) and BG1HOFS/VOFS HDMA writes (channel 1) all arrive **at scanline 48, not 47**. So scanline 47 renders Mode 7 with stale Mode 1 register values + uninitialized M7 matrix.
3. Reference image clearly has Mode 7 horizon at row 47 with the "correct" first-scanline values. Our row 48 values appear to match reference's row 47 conceptually.

## The contradiction

The trace strongly suggests Mode 7 setup arrives one scanline late, but the obvious fix (shift HDMA earlier) breaks things universally. Two facts that must be simultaneously true:

- **Reference's row 47 has correct Mode 7 setup** (BG1HOFS=1008, M7A-D=initial perspective values)
- **Reference's COLDATA per-scanline gradient matches the OLD HDMA timing** (`render(N)` sees `byte_{N-1}`)

These can't both be true under a single uniform HDMA timing. Possibilities:

1. F-Zero's IRQ at V=47 writes M7 setup *directly* (not via HDMA), but our trace shows BG1HOFS=224 at y=47, contradicting a direct $210D write. Need to dump the actual handler code (currently we only see the prologue ending in `JMP ($0041)`; the real handler is at the address stored at $0041 in WRAM).

2. Real-hardware HDMA semantics differ from Anomie's algorithm in some subtle way — e.g., entry transitions on non-repeat entries fetch+transfer atomically on the same scanline, while repeat entries don't. Our impl treats them uniformly.

3. The Mode 7 affine math itself produces subtly wrong VRAM coordinates for "first-scanline-after-IRQ" cases. The smooth-gradient pattern is consistent with this.

## What to investigate next

In rough priority order:

1. **Dump the actual IRQ handler at the address stored in `$0041`** to see if it writes M7 registers directly. This either confirms or rules out hypothesis 1 above.

2. **Compare per-scanline M7A-D values against a known-good emulator** (bsnes-plus, Mesen-S). If our values match at every scanline but the rendering still differs, the bug is in our affine math, not the register state.

3. **If the bug IS in affine math**: look for off-by-one in the "first scanline" case. The smooth gradient could come from an accumulator that's initialized one step off, then walks correctly thereafter.

4. **Possibility: 16-bit overflow in `M7B*SCREEN.Y`**. With M7B=-1280 and SCREEN.Y=224, product is -286720, fits in i32 fine. But check intermediate i16 truncation paths in `get_vram_coords`.

## Investigation tooling in place

- `Ppu::scanline_trace` captures per-scanline register state including a dedicated Mode-7 line (m7a/b/c/d/x/y, m7hofs/vofs, m7sel) when bg_mode==7. Cleared at y=1, surfaces in debug_dump.txt via P (pause) + D (dump).
- IRQ fire log captures (fire_scanline, vtime, htime) for each IRQ during the last complete frame.
- HDMA channel state dump shows dmap, bbad, src/das pointers, line_ctr, do_xfer for all 8 channels.
- F key dumps current frame buffer to PNG with canonical 5→8-bit color conversion (so diffs are trustworthy).

## What this session did NOT change

The pending uncommitted Mode 7 affine formula changes in `src/ppu/mode_7/mod.rs` (the spec-literal `& ~0x3F` masks per partial product + the `clip13` AND-NOT-1C00h variant) match `ppu.md:297-302` exactly and are an improvement over the previous matrix-form formula. Build is clean. They did not by themselves resolve the gradient, but they should be committed since they match spec.

## Files touched this session

- `src/ppu/stat78.rs` — new (STAT78 bitfield)
- `src/ppu/mod.rs` — STAT78 wiring; Mode 7 scanline_trace addition; `toggle_interlace_frame_counter` helper
- `src/super_nintendo/mod.rs` — IRQ-match-after-increment ordering; `toggle_interlace_frame_counter` call at frame end
- `src/memory/addresses.rs` — STAT78 constant
- `src/app/mod.rs` — frame dump 5→8-bit color conversion
- `src/ppu/mode_7/mod.rs` (uncommitted) — spec-literal affine formula
