# PPU Pixel Pipeline — How One Pixel Gets Rendered

## Prerequisites
- [PPU Overview](ppu-overview.md) — BG modes, layer basics
- [System Overview](system-overview.md) — CPU/PPU separation

## The Big Picture

The PPU renders 256 pixels per scanline, 224 scanlines per frame, 60 frames per second. For **each individual pixel**, it runs through the same pipeline:

```
Sample all layers → Pick the winner → Look up the color → Apply brightness → Send to TV
```

## Step 1: Sample Every Layer

For a given pixel position (x, y), the PPU independently asks each enabled layer: "do you have something here?"

```
"What does BG1 look like at pixel (40, 100)?"  →  blue tile, priority=1
"What does BG2 look like at pixel (40, 100)?"  →  transparent (nothing there)
"What does BG3 look like at pixel (40, 100)?"  →  green HUD letter, priority=1
"Any sprite covering pixel 40 on this line?"   →  red character, priority=2
```

Each answer is either **transparent** (nothing at this pixel) or a **CGRAM index + priority value**.

- BG layers have a **1-bit priority** (0 or 1), set **per-tile** in the tilemap entry (bit 13)
- Sprites have a **2-bit priority** (0–3), set **per-sprite** in OAM

## What is a tile?

A tile is a small grid of pixels — the fundamental building block. Base size is **8x8 pixels**.

The SNES can also do **16x16 tiles** (controlled by bits 4–7 of BGMODE register $2105), but a 16x16 tile is really four 8x8 tiles arranged in a 2x2 square:

```
Tile number N in the tilemap becomes:

  ┌─────────┬─────────┐
  │  N      │  N+1    │
  │  (8x8)  │  (8x8)  │
  ├─────────┼─────────┤
  │  N+16   │  N+17   │
  │  (8x8)  │  (8x8)  │
  └─────────┴─────────┘
```

## Step 2: Priority Resolution

The BG mode (bits 0–2 of $2105) determines a **fixed priority ordering** that interleaves BG layers and sprites. The PPU walks this list front-to-back and picks the **first non-transparent** entry.

### Mode 1 example (front → back):

```
 Check #   Layer      Priority    "Is this pixel non-transparent?"
 ──────────────────────────────────────────────────────────────────
    1.      Sprite     pri=3       no  → keep going
    2.      BG1        pri=1       no  → keep going
    3.      BG2        pri=1       no  → keep going
    4.      Sprite     pri=2       YES → WINNER! Stop here.
    5.      BG1        pri=0       (never reached)
    6.      BG2        pri=0       (never reached)
    7.      Sprite     pri=1       (never reached)
    8.      BG3        pri=1       (never reached)
    9.      Sprite     pri=0       (never reached)
   10.      BG3        pri=0       (never reached)
   11.      Backdrop                (never reached)
```

Key observations:
- **Sprites appear 4 times** in the list (once per priority level 0–3)
- **BG layers appear twice** (once for priority=1, once for priority=0)
- **Backdrop** (CGRAM color 0) is always last — wins only if everything else is transparent
- **BG3 priority boost** (BGMODE bit 3, Mode 1 only): BG3 pri=1 tiles jump to the very front, even ahead of sprites — used for HUDs

### Why multiple BG layers exist

Multiple layers let games create **parallax scrolling** cheaply. Each layer has its own scroll registers:

- **BG1** = ground/platforms (scrolls with camera)
- **BG2** = distant mountains (scrolls at half speed)
- **BG3** = HUD text (doesn't scroll)
- **Sprites** = player, enemies, items (positioned freely)

The CPU sets up tilemaps once, then just updates a few scroll values per frame. The PPU does all the rendering work — they're physically separate chips running simultaneously.

## Step 3: Color Lookup

The winning layer gives us a **CGRAM index** (0–255). The PPU looks it up in CGRAM — a 256-entry palette of 15-bit colors:

```
CGRAM entry = 15-bit value:

  Bit  14  13  12  11  10   9   8   7   6   5   4   3   2   1   0
       |_____________| |_____________| |_____________|
            Blue            Green           Red
          (0–31)           (0–31)          (0–31)
```

5 bits per channel = 32 shades each = **32,768 possible colors**.

## Step 4: Master Brightness

INIDISP register ($2100) has a 4-bit brightness value (0–15). Every pixel gets dimmed:

```
final_channel = channel * (brightness + 1) / 16
```

- Brightness 15 (normal): no change
- Brightness 0: everything is black
- Games use this for fade-in / fade-out effects

## Complete Example

```
Pixel (40, 100) in Mode 1:

  1. SAMPLE        BG1 → index 5, pri=1
                   BG2 → transparent
                   BG3 → index 68, pri=0
                   OBJ → transparent

  2. PRIORITY      Walk the list front-to-back...
                   BG1 pri=1 is non-transparent → WINNER
                   Result: CGRAM index 5

  3. COLOR LOOKUP  CGRAM[5] → RGB (12, 24, 8)

  4. BRIGHTNESS    brightness = 15
                   (12×16/16, 24×16/16, 8×16/16) → (12, 24, 8)

  5. → Send to TV
```

Repeat 256 times across × 224 scanlines down × 60 frames per second.

## Optional stages (not covered here)

Between steps 2 and 3, two optional systems can modify the result:

- **Windowing** — rectangular regions that can mask out layers
- **Color math** — blending the main screen with a sub screen (add/subtract)

These are layered on top of the same core pipeline.

## Hardware context

The PPU is a **physically separate chip** (actually two: PPU1/5C77 and PPU2/5C78). It runs simultaneously with the CPU. VRAM is privately connected to the PPU — the CPU accesses it indirectly through registers ($2118/$2119).

Scanline-level emulation accuracy works for ~95%+ of games because most games only change PPU registers during VBlank or via HDMA (between scanlines), not mid-scanline.

## Reference
- Fullsnes: https://problemkaputt.de/fullsnes.htm — search "SNES PPU BG and OBJ Priorities"
- SNESdev Wiki: https://snes.nesdev.org/wiki/PPU
