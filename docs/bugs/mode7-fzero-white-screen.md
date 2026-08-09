# Mode 7 F-Zero — Debug Notes

## Status: White screen RESOLVED, garbage rendering remains

The white screen was caused by HDMA non-repeat mode not being implemented. After fixing that, F-Zero's Mode 7 track is visible but rendering garbage. The next step is debugging the visual artifacts.

## What was implemented this session

### Mode 7 brightness
- `mode_7_sample` now receives `brightness_factor` and applies `channel * (brightness + 1) / 16` per RGB channel, matching modes 0-6.

### Mode 7 OBJ compositing
- `obj_sample()` is called per pixel with windowing (`is_enabled` using TM/TMW/WOBJSEL/WOBJLOG).
- Priority chain: OBJ pri 3 > BG1 (if opaque) > OBJ pri 2/1/0 > backdrop.
- Winner's CGRAM index is used for the final color lookup.

### HDMA transfer modes 2, 3, 4
- Mode 2: 2 bytes to `bbad`, `bbad` (same register, write twice) — needed for Mode 7 double-write latch registers.
- Mode 3: 4 bytes to `bbad`, `bbad`, `bbad+1`, `bbad+1`.
- Mode 4: 4 bytes to `bbad`, `bbad+1`, `bbad+2`, `bbad+3`.

### HDMA non-repeat mode (THE FIX for white screen)
- After decrementing `hdma_line_counter`, set `hdma_do_transfer = (line_counter & 0x80 != 0)`.
- Repeat entries (bit 7 set): transfer every scanline, advance data pointer each time.
- Non-repeat entries (bit 7 clear): transfer once on first scanline, idle for remaining count.
- Without this, non-repeat entries consumed `count * bytes` from the table instead of `1 * bytes`, causing the data pointer to overshoot into zeros.

### PPU multiply ($2134-$2136)
- `Mode7::multiply_result` computed on M7B writes: `m7a * (value as i8 as i32)`.
- MPYL/MPYM/MPYH return bytes 0/1/2 of the 24-bit result.
- Note: F-Zero does NOT use this for its Mode 7 table computation (no reads observed). Implemented for correctness — other games may use it.

### PaletteBase
- Extended match from `1..=3` to `1..=7` so Mode 7 doesn't panic.

## Current state: garbage rendering

The Mode 7 track is now visible (no longer white) but the output is garbled. Possible causes:
1. **Affine transform math** — the origin calculation or matrix multiply might have precision or sign issues
2. **VRAM data interpretation** — Mode 7 interleaved layout (tilemap at even bytes, pixel data at odd bytes) might have addressing bugs
3. **Screen Y coordinate** — `get_screen_flips` receives the V counter (1-224) but Mode 7 SCREEN.Y might need to be 0-based (V-1)
4. **13-bit origin clipping** — the spec says `(HOFS-X)` should be masked/sign-extended to 13 bits; the current code does raw i32 subtraction
5. **HDMA table data** — the game's precomputed tables in WRAM might still have partially wrong values if some CPU/IO feature is missing

### Debugging approach
- Capture a frame dump with F key (PPM in docs/bugs/)
- Compare matrix values per scanline against what a reference emulator produces
- Check if the affine transform formula matches fullsnes exactly (especially the origin computation and per-pixel increment optimization)

## Reference: HDMA algorithm (from Anomie's docs)

Per-scanline steps:
1. If `do_transfer`: transfer data, advance pointer
2. Decrement line counter (full byte, including bit 7)
3. Set `do_transfer = (line_counter & 0x80 != 0)` ← **the fix**
4. If `(line_counter & 0x7F) == 0`: read next entry, set `do_transfer = true`

## Reference: SNES HDMA transfer modes

| Mode | Bytes | Pattern | Use case |
|------|-------|---------|----------|
| 0 | 1 | `+0` | Single register (BGMODE, COLDATA) |
| 1 | 2 | `+0, +1` | Two adjacent registers (VRAM addr lo/hi) |
| 2 | 2 | `+0, +0` | Write-twice register (M7A, M7B, etc.) |
| 3 | 4 | `+0, +0, +1, +1` | Two adjacent write-twice registers (M7A+M7B) |
| 4 | 4 | `+0, +1, +2, +3` | Four adjacent registers |

## Reference: Mode 7 priority order (non-EXTBG)

```
OBJ priority 3    (frontmost)
BG1               (if pixel_color != 0)
OBJ priority 2
OBJ priority 1
OBJ priority 0
Backdrop           (CGRAM index 0)
```

## Files modified this session

- `src/ppu/mod.rs` — brightness + OBJ compositing in `mode_7_sample`, MPYL-MPYH reads
- `src/ppu/mode_7/mod.rs` — `multiply_result` field, `read()` method, multiply on M7B write
- `src/memory/bus.rs` — HDMA transfer modes 2-4, HDMA non-repeat fix
- `src/memory/addresses.rs` — MPYL/MPYM/MPYH constants
- `src/ppu/palette_base.rs` — extended mode range to `1..=7`
