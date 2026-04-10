# LttP Name Screen — Cursor Off By One Character

## Symptom
On the "Register Your Name" screen, the cursor appears one character to the RIGHT of what gets selected. E.g., cursor visually on H selects G. This is a consistent 16-pixel (one character cell) offset between the sprite cursor position and the BG3 character grid.

## Layer Architecture (confirmed 2026-04-10)
- **BG1**: Frame/border + dark fill (tile 0x7F) inside the grid area. NOT the character grid.
- **BG3**: Character letters (tiles 000-009 = A-J, etc.) + tile 0xA9 spacers between them. This IS the character grid.
- **OBJ**: Horizontal solid line (sprites 0-25) + cursor indicator (sprite 26, tile 0x29)
- **BG mode**: Mode 1, BGMODE=0x09 (BG3 priority boost), TM=0x15 (BG1 + BG3 + OBJ)

## Root Cause: 16-pixel misalignment between sprite cursor and BG3 grid

### Sprite position
- Sprite 26 at **x=31** for initial cursor position (position 0)
- Step = +16 per cursor movement (31 → 47 → 63 → 79 → 95 → 111 → 127...)

### BG3 character grid positions (BG3HOFS=0)
- Characters at tilemap columns 2, 4, 6, 8, 10, 12, 14, 16, 18, 20...
- Screen pixel positions: A@x=16, B@x=32, C@x=48, D@x=64, E@x=80, F@x=96, G@x=112, H@x=128

### The mismatch
- Cursor position 0: sprite at x=31, character A at x=16-23 → cursor is 8px past A, visually on B
- Cursor position 6: sprite at x=127, character G at x=112-119 → cursor visually on H
- Selecting at position 6 gives G → "cursor on H, selects G"

### Confirmed by hack test
- Subtracting 16 from BG3 horizontal offset in rendering code **fixes the cursor alignment**
- This shifts the character grid right so A lands at x=32, aligning with cursor at x=31
- But breaks the rest of the screen (global shift)

## What's been ruled out

### BG1 scroll is NOT the issue
- BG1HOFS = 0 is correct — characters are NOT on BG1
- BG1 is the border frame only

### BG3HOFS = 0 is intentional
- Scroll shadow at DP $E4/$E5 is 0
- NMI handler writes $E4/$E5 to $2111 (BG3HOFS) every frame
- **116 writes to $E4/$E5 captured from boot to name screen — ALL are 0x00**
- The game never sets BG3HOFS to non-zero

### Tilemap data is written to correct VRAM addresses
- BG3 tilemap at VRAM 0x6000 (BG3SC=0x63, 64x64 tilemap)
- VRAM write log shows: clear pass (0x7F), spacer fill (0xA9), then character tiles written to columns 2, 4, 6...
- The game intentionally places characters at those columns
- Tilemap is written during screen initialization, NOT during per-frame NMI

### Font data is correctly aligned
- Tile 000 renders as A, tile 002 as C (verified via pixel dump)
- No tile index offset

### NMI handler disassembly (at $80C9)
- Scroll register writes at $8188+:
  - BG1HOFS from WRAM $0120/$0121
  - BG1VOFS from $0124/$0125
  - BG2HOFS from $011E/$011F
  - BG2VOFS from $0122/$0123
  - BG3HOFS from DP $E4/$E5
  - BG3VOFS from DP $EA/$EB
- All scroll shadows are 0 — confirmed by trace
- Handler control flow: APU comm → INIDISP=0x80 → HDMA off → DMA subroutines ($89E0, $83D1) → PPU register updates → scroll writes → BGMODE → INIDISP=0x0F

### Hardware multiply/divide
- Implemented ($4202-$4206 write, $4214-$4217 read)
- Did NOT fix this bug

### VRAM reads
- Were completely missing ($2139/$213A unhandled)
- Implemented: prefetch buffer, read_data_lo/hi, prefetch on VMADDH write, address increment on read
- Did NOT fix this bug

## Likely root cause (unconfirmed)
Either:
1. **The sprite position computation produces x=31 when it should produce ~x=15** — the game's cursor code adds 16 extra pixels, possibly due to reading a wrong value from an unimplemented or incorrectly-stubbed register during initialization
2. **The tilemap should have characters at columns 4, 6, 8... instead of 2, 4, 6...** — something during initialization shifts the tilemap data 2 entries earlier than intended

Both explanations produce the same visual result. The root cause is likely an unimplemented hardware feature or register that the name screen initialization code depends on. Candidates to investigate:
- CPU instruction edge cases (addressing modes, flag handling)
- Unimplemented I/O registers read during init (returning 0 instead of expected value)
- DMA transfer modes 3-7 (only 0-2 fully implemented)
- VRAM address translation (VMAIN bits 3-2, not implemented)
- Block move instructions (MVN/MVP) if used for OAM buffer population

## Code changes made during investigation
- **DMA A1T/DAS update**: after DMA transfer, A1T now advances and DAS is set to 0 (matching real hardware)
- **DMA bank wrapping**: source address wraps within bank via A1T u16 wrapping (no cross-bank DMA)
- **VRAM read support**: prefetch buffer, RDVRAML ($2139), RDVRAMH ($213A) with proper address increment
- **Frame trace system**: T key (while paused) runs one traced frame, writes to `docs/bugs/frame_trace.txt`
- **Various debug outputs in trace**: NMI handler hex dump, BG3 tilemap dump, sprite 26 OAM data, DMA channel configs, VRAM write log, WRAM watchpoints

## Debug infrastructure
- `D` key (while paused): CPU/SPC700/PPU state dump to `docs/bugs/debug_dump.txt`
- `F` key (while paused): frame buffer PPM screenshot to `docs/bugs/`
- `T` key (while paused): full frame trace to `docs/bugs/frame_trace.txt`
- `P` key: pause/unpause emulation
