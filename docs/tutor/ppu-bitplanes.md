# PPU Tile Graphics & Bitplanes

## Prerequisites
- [PPU Overview](ppu-overview.md) — BG modes, VRAM structure

## The Core Problem

Storing each pixel as a full byte (0–255) wastes space. Most layers only need 16 colors (4 bits/pixel). The SNES uses bitplanes to pack multiple pixels tightly.

| Color depth | Bits/pixel | Bytes/tile | Colors |
|-------------|-----------|------------|--------|
| 2bpp        | 2         | 16 bytes   | 4      |
| 4bpp        | 4         | 32 bytes   | 16     |
| 8bpp        | 8         | 64 bytes   | 256    |

## What's a Bitplane?

Instead of storing each pixel's full color index together, the SNES stores them one bit per plane.

For a 2bpp tile (2 planes), pixel color index assembly:
```
Plane 0 (LSB):   0    1    0    1    0    0    1    1
Plane 1 (MSB):   0    0    1    1    0    1    1    0
Color index:     00   01   10   11   00   10   11   01
```

color_index = (plane1_bit << 1) | plane0_bit

## VRAM Layout

Each byte stores one row of 8 pixels for one plane (bit 7 = leftmost, bit 0 = rightmost).

### 2bpp tile (16 bytes):
```
Byte offset   Contents
$00           Plane 0, row 0
$01           Plane 1, row 0
$02           Plane 0, row 1
$03           Plane 1, row 1
...
$0E           Plane 0, row 7
$0F           Plane 1, row 7
```
Planes 0+1 interleaved: even byte = plane 0, odd byte = plane 1.

### 4bpp tile (32 bytes):
```
$00–$0F    Planes 0+1  (interleaved, row by row)
$10–$1F    Planes 2+3  (interleaved, row by row)
```

### 8bpp tile (64 bytes):
```
$00–$0F    Planes 0+1
$10–$1F    Planes 2+3
$20–$2F    Planes 4+5
$30–$3F    Planes 6+7
```

## Reconstructing a Pixel

For pixel at column C, row R in a 4bpp tile with base VRAM word address `tile_base`:

1. Read word at `tile_base + R` → low byte = plane 0 row, high byte = plane 1 row
2. Read word at `tile_base + 8 + R` → low byte = plane 2 row, high byte = plane 3 row
3. Extract bit `7 - C` from each byte (bit 7 = leftmost pixel)
4. Assemble: `color = (p3 << 3) | (p2 << 2) | (p1 << 1) | p0`

The `+8` skips 8 word addresses = 16 bytes = the full planes 0+1 section.

## VRAM Word Addresses

VRAM is word-addressed. `read_word(addr)` fetches 2 bytes — one interleaved pair:
- Low byte = plane N, row R
- High byte = plane N+1, row R

This is why `bg_sample` uses word addresses and `+8`/`+16`/`+24` offsets.

### tile_base_multiplier
Tiles are indexed by number; tile_base = tile_number * tile_base_multiplier:
- 2bpp: 8 words/tile → multiplier = 8
- 4bpp: 16 words/tile → multiplier = 16
- 8bpp: 32 words/tile → multiplier = 32

## Color Index → CGRAM

The assembled color index is NOT the final color — it's a palette index into CGRAM.

- **2bpp**: `cgram_index = palette_base + palette_number * 4 + pixel_index`
- **4bpp**: `cgram_index = palette_base + palette_number * 16 + pixel_index`
- **8bpp**: `cgram_index = pixel_index` (direct, 256-color)

Color index 0 is always transparent (don't write this pixel).

## Tilemap Entry Bits (reminder)

```
Bit 15  14  13  12  11  10  9–0
     |   |   |   |___|___|  |
     |   |   |   palette    tile_number (0–1023)
     |   |   |
     |   |   priority (0 or 1)
     |   x_flip
     y_flip
```

## Reference
- Fullsnes: https://problemkaputt.de/fullsnes.htm — "SNES PPU Tile Data"
