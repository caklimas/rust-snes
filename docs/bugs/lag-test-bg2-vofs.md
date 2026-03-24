# Bug: Lag Test — Scanline flickering, sprite corruption, and off-by-one [RESOLVED]

## Symptoms

On the 240p test suite lag test screen, several visual artifacts appeared:

- **Top scanline (y=0)**: alternated between solid white and a 4px black/4px white checkerboard pattern every frame
- **First scanline on other screens**: black instead of matching the background color
- **Last scanline**: missing on all screens
- **Sprite "5" circle**: corrupted with red/blue fragments; red circle offset from blue circle by 1 pixel

## Issues found

### 1. V counter off-by-one (root cause of most symptoms)

The emulator was rendering scanlines 0–223, using y=0 for the first visible line. On real SNES hardware, the PPU's V counter ranges 0–261, and the **visible scanlines are V=1–224**. V=0 is a pre-render line (not visible). The PPU uses the V counter directly in scroll offset calculations: `y_offset = VOFS + V`.

Using y=0 instead of V=1 produced an off-by-one in every vertical scroll calculation:

- **Old (broken)**: first visible line uses `y_offset = VOFS + 0`
- **Fixed**: first visible line uses `y_offset = VOFS + 1` (V counter = 1)

For the lag test specifically, BG2 VOFS = 0xFFFF (-1), stale from menu setup:
- **Old**: `y_offset = 0xFFFF + 0 = 0xFFFF` → tile row 31 (bottom of tilemap, stale menu data) → visible artifact
- **Fixed**: `y_offset = 0xFFFF + 1 = 0x0000` → tile row 0 (correct content) → no artifact

### 2. Sprite sub-tile flip not applied for multi-tile sprites

For multi-tile sprites (16x16, 32x32, etc.), x_flip and y_flip only flipped the **pixel within each 8x8 tile** but did not flip **which tile** within the sprite grid. A y-flipped 16x16 sprite would flip pixels inside each tile but still read from the wrong tile row/column, producing scrambled fragments.

**Fix**: after computing `sub_tile_col` and `sub_tile_row`, flip them when the corresponding flag is set: `flipped = (tiles_per_axis - 1) - original`, where `tiles_per_axis = sprite_size / 8`.

### 3. Sprite Y position off by 1 after V counter fix

After fixing the V counter for BG layers, sprites appeared shifted up by 1 pixel. Sprite OAM Y coordinates are **screen-relative** (0-based), not V-counter-based. A sprite at Y=0 should appear on the first visible scanline (V=1).

**Fix**: `obj_sample` uses `(y - 1)` for the tile_row calculation: `tile_row = ((y - 1) as u8).wrapping_sub(low.y)`.

## Investigation findings

VRAM watchpoints and DMA logging during the investigation confirmed:
- The tile data at BG2 char_base 0x6000 (tile 0 = 0xF0/0x0F) is **not corruption** — it's legitimate game tile graphics written via DMA during vblank
- BG1 tile 0 at char_base 0x0000 is intentionally blank (transparent)
- CGRAM[1] alternated between 0x7FFF (white) and 0x0000 (black) each frame, causing the visual flicker when the stale BG2 tile was visible
- All VRAM writes passed through during vblank or forced blank — the write guard was working correctly

## Additional fixes applied (correct but not the root cause)

- **INIDISP default**: changed `Display` power-on value from 0x00 to 0x80 (forced blank on), matching real hardware
- **VRAM write guard**: moved from inside `Vram` to `Ppu::write()`, checking `!rendering_active || forced_blank()` — allows writes during forced blank regardless of scanline
