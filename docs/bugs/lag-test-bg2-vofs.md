# Bug: Lag Test — Scanline 0 flickering + edge artifacts

## Symptoms

On the 240p test suite lag test screen, three visual artifacts appear:

- **Top scanline (y=0)**: alternates between solid white and a 4px black/4px white checkerboard pattern every frame
- **Left/right edges (x=0, x=255)**: black pixels across all scanlines that flicker
- **"5" circle**: corrupted with red/blue fragments (likely a separate sprite rendering issue)

## What we've confirmed

### PPU state on lag test screen
- `bg_mode = 1`, `TM = 0x13` (BG1 enabled, BG2 enabled, OBJ enabled)
- `bg2_vofs = 65535` (0xFFFF = -1), `bg2_hofs = 0`
- BG2VOFS is **not written** during the lag test — the value is stale from menu screen init
- The game intentionally writes 0xFFFF to BG2VOFS during menu setup (two byte-pair writes: 0x00/0x00 then 0xFF/0xFF)
- HDMA is **not active** on the lag test screen (`hdmaen = 0`); the per-scanline $2106 mosaic writes during menus come from CPU code

### Scanline 0 rendering
- With vofs=-1, scanline y=0 reads from tile row 31, pixel row 7 (bottom of tilemap)
- BG2 tilemap entry at (0,31) = `0x0000` → tile number 0
- BG2 char_base = `0x6000`
- **Tile 0 data is corrupted**: all 8 rows contain `0x0FF0` instead of `0x0000`
  - In 2bpp: plane 0 = `0xF0` (`11110000`), plane 1 = `0x0F` (`00001111`)
  - This produces the 4-black/4-white checkerboard pattern at CGRAM index 1
- On frames where BG2 wins priority → checkerboard; on frames where BG1 wins → solid white

### VRAM corruption source
- Tile 0 at `0x6000–0x6007` starts as all zeros (VRAM default)
- During the first frame, something writes `0xFF` to all 16 bytes of tile 0
- These writes happen **during active rendering** (between per-scanline $2106 writes)
- On real SNES hardware, VRAM writes during active rendering (when not in forced blank) are **silently ignored**

### Fix attempt: VRAM write guard
- Added `rendering_active` flag to `Vram` struct
- Set true at scanline 0 (when not forced blank), cleared at scanline 225
- Guard in `write_data_lo`/`write_data_hi` blocks writes when `rendering_active = true`
- **Result: tile 0 is still `0x0FF0` at render time** — the guard is not blocking the writes

## Current state of investigation

The VRAM write guard approach isn't working. Possible reasons:
1. The corrupting writes happen **before** the first scanline boundary (during reset vector execution, before `rendering_active` is ever set based on scanline timing)
2. The writes happen during a window where `rendering_active` transitions (e.g., the flag gets cleared at scanline 225 but the CPU runs many instructions before the next scanline check)
3. There may be a timing issue where the game writes to VRAM between the vblank flag being cleared and the rendering guard being set

## Next steps

1. **Find exactly when the corruption happens**: add a watchpoint on VRAM `0x6000` that also logs `rendering_active` and `current_scanline` to understand why the guard isn't catching it
2. **Alternative fix**: instead of guarding individual writes, check `forced_blank` at write time (`if self.rendering_active && !forced_blank` → block). This requires the PPU's `display` register state to be accessible from `Vram`
3. **Consider**: the corruption may happen during the initial boot sequence before any scanline processing. The real SNES starts in forced blank (INIDISP defaults to 0x80), but the emulator's `Display` struct defaults to 0 (forced blank OFF). Fixing the INIDISP default may solve this

## Separate issue: sprite corruption on "5" circle

Red/blue fragments on sprites are likely a different bug in `obj_sample`. Potential issues found:
- `y_flip` doesn't flip `sub_tile_row` for multi-tile sprites
- Sprites at Y=255 wrapping to y=0 may render with wrong tile data

## Debug infrastructure

- Press **D** key to capture diagnostic data for the next 2 frames (gated by `debug_frames_remaining` counter on `Ppu`)
- Logs go to stderr; pipe with `2>bg2vofs.log`

## Run command

```
cargo run --release -- /Users/christopherk/Desktop/Files/240pTestSuite-SNES-1.09/240pSuite.sfc 2>bg2vofs.log
```
