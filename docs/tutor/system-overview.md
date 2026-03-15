# SNES System Overview

## Prerequisites
None — this is the starting point.

## Three Computers on One Board

The SNES is **three mostly-independent processors** running simultaneously:

```
┌─────────────────────────────────────────────────────────┐
│                     CARTRIDGE (ROM)                      │
└──────────────────────────┬──────────────────────────────┘
                           │
              ┌────────────┴────────────┐
              │      CPU  (65C816)      │
              │   "The coordinator"     │
              │   2.68 / 3.58 MHz       │
              └──┬─────────┬─────────┬──┘
                 │         │         │
          ┌──────┘    ┌────┘         └────┐
          │           │                   │
          ▼           ▼                   ▼
   ┌────────────┐  ┌─────────┐    ┌──────────────┐
   │   WRAM     │  │   PPU   │    │   APU        │
   │  128 KB    │  │ "Artist"│    │  "Musician"  │
   │            │  │         │    │  SPC700 CPU  │
   └────────────┘  │  VRAM   │    │  64KB RAM    │
                   │  64KB   │    │  own program │
                   │  CGRAM  │    └──────────────┘
                   │  OAM    │        ▲
                   └────┬────┘        │
                        │        4 shared bytes
                        ▼        ($2140-$2143)
                   ┌─────────┐   ONLY connection
                   │  TV OUT  │   between CPU & APU
                   └─────────┘
```

### The CPU — "The Coordinator"
The 65C816 runs game logic: reading controller input, managing game state, and telling the other chips what to do. It **cannot draw pixels or play sounds directly** — it writes configuration into registers that the PPU and APU read.

### The PPU — "The Artist"
The PPU has **its own private memory** (VRAM, CGRAM, OAM) that the CPU cannot access directly. The CPU writes to I/O registers ($2100-$213F) to push data into VRAM or configure rendering. The PPU then independently reads its own VRAM every scanline to produce pixels.

### The APU — "The Musician"
A **completely separate computer** — its own CPU (SPC700), its own 64KB of RAM, running its own program. The only connection to the main CPU is **four bytes** of shared registers ($2140-$2143). The CPU uploads a sound program and sample data through those 4 bytes, then the APU runs independently.

## Two Buses

| Bus | What it connects | Address range | Who uses it |
|-----|-----------------|---------------|-------------|
| **A-Bus** | CPU ↔ ROM, WRAM, I/O | Full 24-bit ($000000-$FFFFFF) | CPU, DMA |
| **B-Bus** | CPU ↔ PPU/APU registers | 8-bit ($00-$FF, mapped at $2100-$21FF) | CPU, DMA |

When the CPU writes to $2118 (VRAM data), it puts an address on the A-Bus ($002118) and data on the B-Bus simultaneously. This dual-bus design enables DMA — reading from A-Bus (ROM/WRAM) and writing to B-Bus (PPU) at the same time.

## The Frame Loop

The SNES renders at ~60fps (NTSC). Each frame:

```
Scanline 0   ─┐
  ...          │  ACTIVE DISPLAY (scanlines 0-224)
Scanline 224 ─┘  PPU is rendering — CPU must NOT touch VRAM/OAM/CGRAM
                  HDMA transfers happen each scanline
                  CPU runs game logic

Scanline 225 ─┐
  ...          │  VBLANK (scanlines 225-261)
Scanline 261 ─┘  PPU is idle — CPU CAN freely write to VRAM/OAM/CGRAM
                  NMI fires at scanline 225 → CPU jumps to NMI handler
                  CPU uses DMA to blast new data into VRAM
```

### A typical game frame:
1. **Active display (0-224):** CPU runs game logic (physics, AI, collision). HDMA does per-scanline effects. PPU renders from VRAM.
2. **VBlank (225-261):** NMI fires. CPU uses DMA to upload new tiles, sprites, palettes into VRAM/OAM/CGRAM.
3. **Repeat.**

### Why DMA matters
VBlank is short (~2.4ms). DMA transfers data ~10x faster than CPU byte-by-byte copies. Without DMA, games couldn't update enough VRAM data between frames.

## Key Insight: CPU Speed Varies

The CPU doesn't run at a single speed — it depends on what memory it's accessing:

| Access target | Speed |
|--------------|-------|
| Fast ROM (banks $80-$FF upper) | 3.58 MHz |
| WRAM, slow ROM | 2.68 MHz |
| Joypad I/O | 1.78 MHz |

This is why cartridge headers declare "fast/slow ROM" — it tells the CPU which clock to use.

## Related Topics
- [PPU Overview](ppu-overview.md) — how the PPU builds each scanline
- [Memory Map](memory-map.md) — the 24-bit address space in detail
- [DMA & HDMA](dma-how-it-works.md) — data transfer mechanisms
- [APU / SPC700](apu-overview.md) — the sound subsystem
