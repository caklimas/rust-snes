# Quiz Log

## 2026-03-15 — System Overview

| # | Question | Result | Notes |
|---|----------|--------|-------|
| 1 | When can CPU write to VRAM and why? | Partial | Knew it's VBlank-only, but thought the reason was CPU being busy with game logic. Actual reason: PPU owns VRAM during active display. Also missed forced blank ($2100 bit 7). |
| 2 | How does sound data get into APU RAM? | Correct | Knew the 4-byte shared register channel ($2140-$2143). Didn't recall the handshake protocol details but had the core concept. |
| 3 | What does the CPU do during DMA? | Incorrect | Thought CPU puts address on A-Bus / data on B-Bus. Actually the CPU is fully halted — DMA controller takes over both buses. |

| 4 | DMA vs HDMA — when and how? | Correct | Knew DMA is VBlank bulk uploads, HDMA is per-scanline. Didn't know HDMA's purpose (per-scanline PPU register changes for effects like gradients). |

### Areas to revisit
- PPU ownership of VRAM during active display vs VBlank
- Difference between CPU register writes (A-Bus + B-Bus) and DMA transfers (CPU halted)
- Forced blank (INIDISP bit 7) as an alternative to VBlank for VRAM access
