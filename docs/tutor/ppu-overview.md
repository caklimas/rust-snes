# PPU Overview

## Prerequisites
- [System Overview](system-overview.md) — CPU/PPU/APU architecture, VRAM access timing

## The PPU's Job

The PPU produces 256 pixels per scanline, entirely from its own private memory (VRAM, CGRAM, OAM). The CPU cannot access those memories directly during active display — it sets up data during VBlank, then the PPU runs independently.

## The PPU's Four Private Memories

```
┌─────────────────────────────────────────────────────────┐
│                         PPU                             │
│                                                         │
│  VRAM (64KB)          CGRAM (512B)       OAM (544B)     │
│  ┌───────────────┐   ┌─────────────┐   ┌────────────┐  │
│  │ Tile graphics │   │ 256 colors  │   │ 128 sprite │  │
│  │ (bitplanes)   │   │ 15-bit RGB  │   │ positions  │  │
│  │               │   │             │   │ & attrs    │  │
│  │ Tilemap data  │   │             │   │            │  │
│  │ (1024 words   │   └─────────────┘   └────────────┘  │
│  │  per screen)  │                                      │
│  └───────────────┘                                      │
└─────────────────────────────────────────────────────────┘
```

- **VRAM**: tilemaps (32×32 grids of tile indices + attributes) + tile graphics (bitplane pixel data)
- **CGRAM**: 256 entries of 15-bit BGR555. Index 0 = always transparent (backdrop)
- **OAM**: 128 sprite records — position, tile, palette, flip, priority

## BG Modes

BGMODE ($2105) selects one of 8 modes. Each trades layer count for color depth:

| Mode | BG1      | BG2      | BG3     | BG4     | Notes |
|------|----------|----------|---------|---------|-------|
| 0    | 4-color  | 4-color  | 4-color | 4-color | 4 layers |
| 1    | 16-color | 16-color | 4-color | —       | Most common |
| 2    | 16-color | 16-color | —       | —       | + offset-per-tile |
| 3    | 256-color| 16-color | —       | —       | |
| 4    | 256-color| 4-color  | —       | —       | + offset-per-tile |
| 5    | 16-color | 4-color  | —       | —       | 512px wide (hires) |
| 6    | 16-color | —        | —       | —       | hires + offset-per-tile |
| 7    | 256-color| EXTBG    | —       | —       | Affine transform |

## Rendering Pipeline (per scanline)

```
For each pixel x (0–255):
  │
  ├─ 1. LAYER SAMPLING
  │     For each enabled BG: sample tile at (x + scroll, y + scroll)
  │     For sprites: find first sprite covering pixel x
  │     Each returns: (cgram_index, priority_bit)
  │
  ├─ 2. PRIORITY RESOLUTION
  │     Combine all samples using mode's priority table
  │     Pick frontmost non-transparent layer → CGRAM index
  │
  ├─ 3. COLOR MATH (optional)
  │     Blend main screen color with sub-screen (add or subtract)
  │
  └─ 4. MASTER BRIGHTNESS
        output = channel * (brightness+1) / 16
```

## Priority Resolution

Each BG tile has a priority bit (tilemap entry bit 13). Sprites have a 2-bit priority (0–3). The PPU interleaves layers based on these bits.

### Mode 1 priority (front → back):
```
OBJ.3  BG1.1  BG2.1  OBJ.2  BG1.0  BG2.0  OBJ.1  BG3.1  OBJ.0  BG3.0  Backdrop
```

When BGMODE bit 3 (BG3 priority boost) is set:
```
BG3.1  OBJ.3  BG1.1  BG2.1  OBJ.2  BG1.0  BG2.0  OBJ.1  OBJ.0  BG3.0  Backdrop
```
BG3 priority=1 tiles jump to the very front — used for HUDs/status bars.

### Full priority table (all modes, front→back):

| Mode 0 | Mode 1 | Mode 2 | Mode 3 | Modes 4–6 |
|--------|--------|--------|--------|-----------|
| OBJ.3  | (BG3.1)| OBJ.3  | OBJ.3  | OBJ.3     |
| BG1.1  | OBJ.3  | BG1.1  | BG1.1  | BG1.1     |
| BG2.1  | BG1.1  | BG2.1  | BG2.1  | BG2.1     |
| OBJ.2  | BG2.1  | OBJ.2  | OBJ.2  | OBJ.2     |
| BG1.0  | OBJ.2  | BG1.0  | BG1.0  | BG1.0     |
| BG2.0  | BG1.0  | BG2.0  | BG2.0  | BG2.0     |
| OBJ.1  | BG2.0  | OBJ.1  | OBJ.1  | OBJ.1     |
| BG3.1  | OBJ.1  | OBJ.0  | OBJ.0  | OBJ.0     |
| BG4.1  | BG3.1  | BG1.0* | BG1.0* | BG1.0*    |
| OBJ.0  | OBJ.0  | BG2.0* | BG2.0* | BG2.0*    |
| BG3.0  | BG3.0  | Backdrop| Backdrop| Backdrop |
| BG4.0  | Backdrop|       |        |           |
| Backdrop|       |        |        |           |

*These are lower in the stack in modes 2+

## Sub-topics to explore

- [Tile Graphics & Bitplanes](ppu-bitplanes.md) — how VRAM stores pixel data
- [Tilemaps & Scrolling](ppu-tilemaps.md) — 32×32 screens, HOFS/VOFS
- [Color Math](ppu-color-math.md) — sub-screen blending
- [Windowing](ppu-windowing.md) — hardware clip windows
- [Mode 7](ppu-mode7.md) — affine transforms
- [Sprites (OAM)](ppu-sprites.md) — tile addressing, 9-bit X, size table
- [Offset-Per-Tile](ppu-offset-per-tile.md) — per-column parallax in Modes 2/4/6

## Reference
- Fullsnes: https://problemkaputt.de/fullsnes.htm — search "SNES PPU"
- SNESdev Wiki: https://snes.nesdev.org/wiki/PPU
