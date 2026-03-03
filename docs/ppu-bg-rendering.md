# SNES BG Rendering

The PPU renders one scanline at a time, left to right. For each of the 256 pixels on a scanline,
it asks each enabled BG layer "what color is your pixel here?" then composites the results.

## Pipeline for a single BG pixel

### Step 1: Find which tile covers this pixel

Every BG layer is a grid of 8×8 tiles (or 16×16 — ignore that for now). The scroll registers
shift the view into that grid. For pixel at screen position `(x, y)`:

```
tile_x = (x + hofs) / 8
tile_y = (y + vofs) / 8
pixel_x_within_tile = (x + hofs) % 8
pixel_y_within_tile = (y + vofs) % 8
```

The tilemap can be 32 or 64 tiles wide/tall (from `mirror_size` in BGxSC). Wrap `tile_x` and
`tile_y` to stay within bounds.

### Step 2: Read the tilemap entry from VRAM

The tilemap starts at the VRAM word address from `BGxSC`. Each entry is a **16-bit word**:

```
Bits 9–0:   tile number (which graphic to draw)
Bits 12–10: palette number
Bit 13:     tile priority (0 or 1)
Bit 14:     X flip
Bit 15:     Y flip
```

The tilemap entry address:

```
entry_addr = tilemap_base + tile_y * tilemap_width + tile_x
```

Apply X/Y flip to `pixel_x_within_tile` / `pixel_y_within_tile` if those bits are set.

### Step 3: Read the character data from VRAM

The character (tile graphics) base address comes from `BG12NBA` / `BG34NBA`. In **Mode 1**,
BG1 and BG2 are **4bpp** (4 bits per pixel = 16 colors per tile), BG3 is **2bpp** (4 colors
per tile).

For **4bpp**, each 8×8 tile occupies **32 bytes** in VRAM, laid out in two interleaved bitplane pairs:

```
Bytes 0–15:  rows 0–7, bitplanes 0 and 1
             (interleaved: row0 plane0, row0 plane1, row1 plane0, row1 plane1, ...)
Bytes 16–31: rows 0–7, bitplanes 2 and 3 (same interleaving)
```

To get a pixel's 4-bit color index, read the two bytes for that row from each pair and extract
the bit at `pixel_x_within_tile`.

For **2bpp**: same idea but only 16 bytes per tile (one bitplane pair, no second pair).

### Step 4: Look up the color in CGRAM

The 4-bit pixel value (0–15) is an index into a palette. The palette number from the tilemap
entry selects which group of 16 colors:

```
cgram_index = palette_number * 16 + pixel_color_index
```

**Color index 0 is always transparent** — if the pixel value is 0, this pixel is see-through
and you fall through to the next layer.

CGRAM stores BGR555 as 16-bit values. That's your output pixel.

### Step 5: Priority and compositing

Each layer produces a pixel (or transparent). For an initial implementation, use a simple
painter's algorithm — render BG3 first, then BG2, then BG1, then sprites, each overwriting the
previous unless transparent. This isn't fully correct but gets you to visible output. Add proper
priority later.

## Output

```
framebuffer[scanline * 256 + x] = u16 BGR555 color
```

## Implementation order

1. Check `display.forced_blank()` first — if set, fill the scanline with black and return early.
2. Implement Mode 1, BG1 only, no flip, no priority.
3. Add BG2 and BG3.
4. Add X/Y flip.
5. Add proper priority compositing.
6. Add sprites (OAM).
