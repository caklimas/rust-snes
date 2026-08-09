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

## Corrected understanding (2026-04-09)
- The vertical dashed line is CORRECT and should stay fixed — it's the backdrop color (232,208,144) showing through transparent gaps in BG3 tile 0xA9
- It's the **character grid (BG1) that should scroll horizontally** when pressing left/right, not the cursor line
- The off-by-one (selects G when visually at H) is because BG1 HOFS is stuck at 0 — the grid should shift to align the correct letter under the fixed cursor line

## Root cause investigation
- **BG1 HOFS is always 0** — confirmed across multiple debug dumps, never changes
- **Game never writes to $210D (BG1HOFS)** — traced one full frame with write logging, zero writes captured
- **Window registers all zero** — not used on this screen
- **HDMA not active** — HDMAEN always 0x00
- **PPU rendering is correct** — pixel trace at (131,120) confirms all layers return None (backdrop shows through as expected)
- **NMI handler IS running** — sprites update correctly (horizontal line moves with up/down, sprite 26 X updates)
- **Hardware multiply/divide was missing** — implemented ($4202-$4206 write, $4214-$4217 read), but did not fix this bug
- **Conclusion**: the game's NMI handler reaches OAM DMA (sprites update) but never reaches the code that writes BG1HOFS. Some game logic condition is not being met, likely due to a read returning an incorrect value from an unimplemented or stubbed register

## Next steps
- Trace what the NMI handler actually executes — disassemble the code at the NMI vector and step through to find where it branches away from the scroll update
- Check if the game reads any I/O register ($4200-$5FFF) that returns 0 from the `InputOutput` catch-all and uses it for a branch decision
- Compare against a reference emulator (bsnes trace log) to find the divergence point

## Debug infrastructure added
- `D` key (while paused) writes CPU/SPC700/PPU state to `docs/bugs/debug_dump.txt`
- `F` key (while paused) writes frame buffer PPM to `docs/bugs/`
- `P` key pauses/unpauses emulation
