# Mode 7 F-Zero White Screen — Debug Notes

## Symptom

F-Zero title screen: the lower three-quarters of the screen renders as solid white. A YouTube reference shows it should display a Mode 7 perspective track demo.

## What was implemented this session

### Mode 7 brightness
- `mode_7_sample` now receives `brightness_factor` and applies `channel * (brightness + 1) / 16` per RGB channel, matching modes 0-6.

### Mode 7 OBJ compositing
- `obj_sample()` is called per pixel with windowing (`is_enabled` using TM/TMW/WOBJSEL/WOBJLOG).
- Priority chain: OBJ pri 3 > BG1 (if opaque) > OBJ pri 2/1/0 > backdrop.
- Winner's CGRAM index is used for the final color lookup.

### HDMA transfer modes 2, 3, 4
- Mode 2: 2 bytes to `bbad`, `bbad` (same register, write twice) — needed for Mode 7 double-write latch registers.
- Mode 3: 4 bytes to `bbad`, `bbad`, `bbad+1`, `bbad+1`.
- Mode 4: 4 bytes to `bbad`, `bbad+1`, `bbad+2`, `bbad+3`.

### PaletteBase
- Extended match from `1..=3` to `1..=7` so Mode 7 doesn't panic.

## What we confirmed

1. **F-Zero uses HDMA channels 4-7 with transfer mode 2** to write M7A ($211B), M7B ($211C), M7C ($211D), M7D ($211E) per scanline. This creates the perspective scaling effect.
2. **Writes DO reach the PPU** — a trace inside `ppu.write()` at the `M7SEL..=M7Y` arm showed non-zero values arriving (e.g. `0x2F`, `0xA3`, `0xC0`).
3. **But matrix values are zero at render time** — traces inside `mode_7_sample` at scanlines 2, 10, and 112 all showed `M7A=0 M7D=0`.
4. **HDMA runs before render_scanline** (`super_nintendo/mod.rs:66-67`), so timing order is correct.
5. **No code runs between HDMA and render** — they're back-to-back in the same `if` block.

## The mystery

HDMA writes arrive at the PPU with non-zero values, but by the time `mode_7_sample` reads the matrix, all values are zero. Since both the write path (`ppu.write() -> mode_7.write() -> get_affine_value()`) and the read path (`mode_7_sample -> self.mode_7.affine_matrix.m7a`) operate on the same `Ppu.mode_7` struct, the values should persist.

## Leading theories

### 1. The non-zero writes are from VBlank CPU code, not HDMA
The `[PPU M7]` trace was NOT filtered by scanline — it captured all writes across the entire frame. The non-zero values might come from CPU writes during VBlank (the game setting up initial matrix values), while the HDMA during active rendering might actually be writing zeros. The HDMA table data itself could be at the wrong address.

**How to verify:** Add a trace inside the HDMA mode-2 handler (bus.rs, the `2 =>` arm) that prints the source address and data bytes for ch4 only. Compare the source address against what the ROM actually contains at that location.

### 2. HDMA table pointer drift from non-repeat mode mishandling
The emulator currently treats ALL HDMA entries as repeat mode (always transfers). If F-Zero's HDMA tables contain non-repeat entries (bit 7 = 0 in the line count byte), the emulator would advance the data pointer every scanline instead of once, consuming the table too fast and overshooting into garbage/zeros.

The initial line counter for ch4-7 is 181 (0xB5) — bit 7 set = repeat, count = 53 scanlines. After 53 scanlines the channels disappear from the trace (next entry byte is 0, terminating the table). But F-Zero's track should have ~200 scanlines of data.

**How to verify:** When the line counter expires and the next entry byte is read (bus.rs around line 268), log the address being read and the value. If it's 0 but shouldn't be, the pointer has drifted.

**How to fix (if confirmed):** Implement non-repeat mode properly. After a transfer on a non-repeat entry, set `hdma_do_transfer = false`. The pointer should only advance by `bytes_consumed` on the one scanline where the transfer happens, then stay put until the counter expires.

### 3. HDMA table address is wrong
The HDMA channels' `a1b` (bank) and `a1t` (table start address) might not point to the correct location in ROM. If the address is slightly wrong, the table data could be all zeros (uninitialized WRAM or wrong ROM region).

**How to verify:** At `init_hdma`, log `a1b`, `a1t`, and the first few bytes of the table for ch4. Cross-reference with the ROM contents at that address.

## How to continue debugging

The most productive next step is a single trace in the HDMA mode-2 code path (`bus.rs`, the `2 =>` arm) that fires for ch4 only, on the first 3 scanlines:

```rust
// Inside the 2 => arm, after both writes:
if i == 4 && self.dma_channels[4].hdma_line_counter >= 0xB3 {
    let lo = self.read(address);
    let hi = self.read(address + 1);
    eprintln!(
        "[HDMA2] ch=4 src={:#08X} lo={:#04X} hi={:#04X} ctr={}",
        address, lo, hi,
        self.dma_channels[4].hdma_line_counter,
    );
}
```

Note: `self.read(address)` is called again here just for logging — the actual data was already read and written above. This tells you:
- **src address**: where the HDMA is reading from. Verify this is valid ROM/WRAM.
- **lo/hi bytes**: the actual data. If both are 0x00, the table data is wrong (theory 1 or 3).
- **ctr**: which scanline within the entry.

If the data IS non-zero but the matrix is still zero at render, the bug is in `Mode7::get_affine_value()` or the `m7_old` latch. Add a trace there showing the latch state before/after each write.

## Reference: SNES HDMA transfer modes

| Mode | Bytes | Pattern | Use case |
|------|-------|---------|----------|
| 0 | 1 | `+0` | Single register (BGMODE, COLDATA) |
| 1 | 2 | `+0, +1` | Two adjacent registers (VRAM addr lo/hi) |
| 2 | 2 | `+0, +0` | Write-twice register (M7A, M7B, etc.) |
| 3 | 4 | `+0, +0, +1, +1` | Two adjacent write-twice registers (M7A+M7B) |
| 4 | 4 | `+0, +1, +2, +3` | Four adjacent registers |

## Reference: Mode 7 priority order (non-EXTBG)

```
OBJ priority 3    (frontmost)
BG1               (if pixel_color != 0)
OBJ priority 2
OBJ priority 1
OBJ priority 0
Backdrop           (CGRAM index 0)
```

## Files modified this session

- `src/ppu/mod.rs` — brightness + OBJ compositing in `mode_7_sample`, render_scanline passes brightness_factor
- `src/memory/bus.rs` — HDMA transfer modes 2, 3, 4 in `run_hdma_scanline`
- `src/ppu/palette_base.rs` — extended mode range to `1..=7`
