# LttP Name Screen — Vertical Cursor Doesn't Move

## Symptom
On the "Register Your Name" screen, the vertical dotted line (column cursor) does not move visually when pressing left/right. The horizontal solid line (row cursor) moves correctly with up/down. The game logic works — selecting a character picks the correct letter for the actual cursor position, but the visual doesn't match.

The initial cursor position is on H, but selecting a character selects the one to the left (e.g. G on first press).

## What we've confirmed
- **BG mode**: Mode 1, TM=0x15 (BG1 + BG3 + OBJ enabled)
- **BG scroll offsets**: BG1/BG2/BG3 HOFS all 0 and don't change on left/right
- **BG1 tilemap**: Contains the character grid, frame border, and background. Identical before and after pressing left/right — no tilemap updates for the cursor.
- **BG3 tilemap**: Filled with tile 0xA9 everywhere — a background fill layer, not the cursor.
- **HDMA**: HDMAEN is always 0x00. The game never enables HDMA on the name screen. Not used for cursor positioning.
- **OAM data**: 
  - Sprites 0–25: horizontal line at y=131, tile 0x2E, evenly spaced x=24..224 (the solid horizontal line)
  - Sprite 26: tile 0x29, y=88, **X position updates correctly** (79 → 95 → 111 when pressing left/right, +16 per step)
  - The vertical dotted line is visually much taller than one 8x8 or 16x16 sprite

## Full sprite dump results
- Only sprite 26 (tile 0x29, y=88, size=small/8x8) is in the visible Y range
- All other non-zero sprites have y=240 (offscreen)
- The visible dotted vertical line is ~80+ pixels tall — far too large for a single 8x8 sprite
- **Conclusion: the vertical dotted line is NOT sprites.** It's likely rendered using BG3 + window masking.

## Leading theory: Window masking on BG3
- BG3 is filled with tile 0xA9 (same tile everywhere)
- Tile 0xA9 likely contains the dotted pattern
- The game probably uses PPU windows (WH0–WH3, W34SEL/WOBJSEL, TMW/TSW) to mask BG3 to a narrow vertical column
- When the cursor moves left/right, the game updates the window left/right bounds (WH0/WH1 or WH2/WH3)
- **Next step**: trace window register values (WH0–WH3, W12SEL, W34SEL, TMW, TSW) on the name screen and check if they change when pressing left/right

## Sprite 26 (tile 0x29)
- This IS visible and its X updates in OAM, but visually doesn't move
- Might be a separate small cursor indicator (not the dotted line itself)
- Possible it's rendered behind a BG layer or its tile is transparent — investigate separately

## OAM fixes applied during investigation
- **OAM VBlank address reload**: added `reload_address` field and `reset_address()` at scanline 225
- **OAM write buffering**: first byte of low-table writes goes to buffer, committed as word on second byte

## Debug infrastructure added
- `D` key toggles `debug` flag on `SuperNintendo`, triggers one-shot logging at scanline 0
- `HDMAEN` write trace (removed after confirming game doesn't use HDMA here)
