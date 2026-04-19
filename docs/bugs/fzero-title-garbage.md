# F-Zero Title Screen — Mode 1 Garbage Rendering

## Status: NOT fixed. Next session should pick up here.

The F-Zero title screen renders the logo and menu text correctly but shows garbage where the sky gradient and the perspective road should be. See `frame_*.ppm` dumps in this folder for current output; compare against `F-Zero-title.png` (the reference).

## Key discovery (was wrong about Mode 7)

Per-scanline trace proves **F-Zero's title is pure Mode 1 — it never switches to Mode 7.** The Mode 7 "white screen" path was a misdirection. The only per-scanline HDMA effect is **BG1 scroll changing once at y=48** from `(224, 36)` to `(1008, 440)` and staying there. `BGMODE` is constant 0x09 (Mode 1, BG3 priority boost enabled) for every scanline.

The `clip_13` fix in `src/ppu/mode_7/mod.rs:86` is still correct per fullsnes spec but didn't affect this screen.

## What we know from VRAM dumps

- **BG1** char_base 0x6000 (word), tilemap 0x7800, mirror_size 1 (64×32), 8×8 tiles. Almost entirely filled with tile 384 (`0x1D80`, palette 7, priority 0), which is **fully transparent** — char data at word 0x7800..0x780F is all zeros (happens to overlap BG1 tilemap rows 0..3 which are also all zero, so no collision in practice). Sparse non-blank road-marker tiles on row 29.
- **BG2** char_base 0x6000 (shared with BG1!), tilemap 0x7000, mirror_size 1. Scroll constant at (192, 92).
  - Rows 11–14: sky gradient tiles `0x1C01..0x1C0A` — palette 7, **priority 1**
  - Rows 15–16: logo tiles `0x1826`, `0x1836` — palette 0, priority 0 (**these render correctly**)
  - Rows 17+: road tiles `0x1C46`, `0x1C0A` etc — palette 7, **priority 1**
- **CGRAM palette 0** (indices 0..15): sparse, mostly 0x0000/0x7FFF — what the logo uses. Renders fine.
- **CGRAM palette 7** (indices 112..127): real colors — e.g. `45EF 3D20 3CC0 ...` — what sky+road use. Suspected problem area but DATA looks correct.
- **Char data** for tiles 1, 5, 10 (sky), 38 (logo), 70 (road) all look like valid pixel patterns, not garbage.

## Hypothesis (old, partially disproven)

Initial thought: sky + road (palette 7, priority 1) broken, logo (palette 0, priority 0) works. Toggle test showed it's actually both BG1 and BG2 contributing garbage in different regions — so it's not a simple palette/priority-specific bug.

## Spec verification done against fullsnes (important)

After (belatedly) actually fetching fullsnes and verifying our code against the spec, **every register/formula interpretation is correct**. Explicitly verified:

| Check | Spec | Our code |
|---|---|---|
| BG12NBA / BG34NBA | 4 bits per BG, 4K-word steps | `base * 0x1000` ✅ |
| BGxSC | bits 7-2 = base in 1K-word steps; bits 1-0: 0=32×32, 1=64×32, 2=32×64, 3=64×64 | matches ✅ |
| 4bpp tile layout | 32 bytes, bp0 at even bytes 0..14, bp1 at odd bytes, bp2+bp3 at bytes 16..31 same pattern | `get_planes`: word read, low=bp0, high=bp1 ✅ |
| Tile address | `B*4096 + N*16` words for 4bpp | `char_base + tile*16` ✅ |
| Tilemap entry format | bits 9-0 tile, 12-10 palette, 13 priority, 14 x_flip, 15 y_flip | matches ✅ |
| Mode 1 priority order (with + without BG3 boost) | BG3.1a (boost) → OBJ3 → BG1.1 → BG2.1 → OBJ2 → BG1.0 → BG2.0 → OBJ1 → BG3.1b (no boost) → OBJ0 → BG3.0 → backdrop | `priority_resolver::mode_1_sample` matches ✅ |
| CGRAM 4bpp lookup | `palette_number*16 + char_data` | matches ✅ |

So the spec-level interpretation is sound. The bug is NOT in any of these formulas. Stop checking them.

## New primary hypothesis: missing HDMA-to-CGDATA

F-Zero's smooth sky gradient is almost certainly produced by **HDMA writing to $2122 (CGDATA) per scanline**, updating CGRAM colors as the raster advances — a classic SNES technique for smooth gradients that can't be done with static tile colors. Our static BG2 tilemap (tiles 1..10 in a diagonal pattern) gives a striped gradient without per-scanline CGRAM updates, which visually matches what we see.

Similar thinking applies to the road garbage: the game may be HDMA-writing CGRAM or even BGxSC mid-frame, and if our HDMA dispatcher silently drops those targets, we render stale/wrong data.

Our per-scanline trace captures register state at the START of `render_scanline`. It would miss any mid-scanline CGRAM changes (CGRAM isn't in the trace at all), and might miss mid-frame BGxSC changes depending on HDMA ordering.

## Debug infrastructure added this session

All still in place for the next instance:

- **Per-scanline trace buffer** (`Ppu::scanline_trace`) populated each `render_scanline` with bgmode, bg1/bg2 register values, scroll offsets, tm/ts. Cleared at y=1.
- **VRAM dump** in `SuperNintendo::debug_info()`: BG1 tilemap rows 0..3, row 4, row 29; BG2 rows 11..22; char data for tiles 1, 5, 10, 38, 70; CGRAM palettes 0 and 7.
- **Layer toggle keys** in `src/app/mod.rs`:
  - `1` — toggle BG1
  - `2` — toggle BG2
  - `3` — toggle BG3
  - `4` — toggle OBJ (sprites)
  - Stored in `Ppu::debug_disabled_layers` (bit mask; 0 = all enabled by default). Prints new value on each toggle.

## Layer toggle test results

Ran the toggle keys. **Both layers render garbage, in different regions:**

- Press `1` (disable BG1): garbage *directly underneath the logo* disappears. So BG1's sparse non-blank tiles at tilemap row 29 (tiles 32, 48, 64, 80) are rendering incorrectly in the y≈48–80 band.
- Press `2` (disable BG2): garbage at *the bottom of the screen* disappears. So BG2 renders correctly in the middle (road area still looks plausible with BG1 off) but breaks at the bottom where BG2 wraps into its low tile rows (tile_y = 0..7 of BG2 tilemap, which we never dumped).

So it's not a single-layer bug. The road area below the logo *does* render reasonably when BG1 is off, which suggests BG2 rows 17..31 (main road content) are fine. BG1 blanks + BG2 rows 0..7 (post-wrap) is where things fall apart.

## Recommended next steps (in order)

1. **Audit HDMA target dispatch.** In `src/memory/bus.rs` (or wherever HDMA writes go), verify that an HDMA write to $2122 (CGDATA) and $2121 (CGADD) actually lands in `Ppu::cgram` via the same code path as CPU writes. Grep for `CGDATA` / `CGADD` and check the HDMA transfer loop routes B-bus writes through `Bus::write` / `Ppu::write`. If CGRAM HDMA writes are silently dropped, that's almost certainly the bug.

2. **Extend per-scanline trace to snapshot CGRAM deltas.** Add a hash/checksum of CGRAM (or a few key indices like 112, 113, 117, 122) into `Ppu::scanline_trace`. If CGRAM changes per scanline, we'll see it. If HDMA IS writing CGRAM but our impl drops it, CGRAM stays constant in the trace — confirming (1).

3. **Trace HDMA channels each frame.** Add a one-shot dump of all 8 HDMA channel configs (source bank/addr, `bbad` B-bus address, `dmap`, line counter) captured at the start of each frame into `debug_info`. This tells us exactly which PPU registers HDMA is writing without needing to guess.

4. If 1-3 come back clean (HDMA does write CGRAM correctly and CGRAM does change per scanline in the trace), fall back to:
   - Dump BG2 rows 0..10 and 22..31 — see if tile numbers for BG2 bottom wrap are unexpected
   - Dump char data for BG1 tiles 32, 48, 64, 80 — see if the sparse road markers have sane data

The earlier-proposed VRAM dumps are still useful as a fallback but only after (1)–(3) are cleared.

## Files touched this session

- `src/ppu/mode_7/mod.rs` — added `clip_13` to `get_origin_relative_coords` (still correct per spec, independent of this bug)
- `src/ppu/mod.rs` — `scanline_trace: Vec<String>`, `debug_disabled_layers: u8`, `cgram` made pub, layer mask wired into every `is_enabled` call in `mode_0_6_sample`
- `src/super_nintendo/mod.rs` — extensive `debug_info` additions (trace + VRAM dumps)
- `src/app/mod.rs` — Digit1..4 key handlers for layer toggles

## Lesson for next session

Start by reading fullsnes for every hardware fact relevant to the bug BEFORE staring at code. I wasted hours speculating from memory before finally fetching, and every register/formula check turned out to match the spec — so all that speculation was dead-end. The productive signal came from the layer-toggle test and the observation that F-Zero likely uses HDMA-to-CGDATA (which fullsnes confirms is a normal HDMA target).
