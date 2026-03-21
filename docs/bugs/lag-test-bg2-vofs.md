# Bug: Lag Test — BG2 VOFS = 65535 causes black edge artifacts

## Symptoms

On the 240p test suite lag test screen, three visual artifacts appear:

- **Top scanline (y=0)**: alternating black/white pixel pattern that flickers
- **Left/right edges (x=0, x=255)**: black pixels across all scanlines that flicker
- **"5" circle**: corrupted with red/blue fragments (may be a separate issue)

## Root Cause (identified, not yet fixed)

`bg2_vofs` is 65535 (0xFFFF = -1 signed) on affected frames.

With vofs = -1:
- Scanline y=0 renders from the BOTTOM row of the BG2 tilemap (row 255, wrapped from below), which contains black pixels
- The left/right black column artifacts come from black pixels at the left and right edges of the BG2 tilemap (tile_x=0/pixel_x=0, tile_x=31/pixel_x=7)

The flickering means `bg2_vofs` alternates between 65535 and a correct value frame-to-frame.

## What we know about the write formula

`set_vertical_offset` in `src/ppu/mod.rs`:
```rust
fn set_vertical_offset(&mut self, number: u8, value: u8) {
    let offset = ((value as u16) << 8) | (self.bg_old as u16);
    // ...
    self.bg_old = value;
}
```

For vofs = 0xFFFF: both `value = 0xFF` and `bg_old = 0xFF` at time of write.

The formula itself is correct per SNES spec. The bug is in **what values are being written** — either:
1. HDMA is feeding incorrect data to BG2VOFS
2. `bg_old` is polluted from a previous register write before the BG2VOFS write

## Debug state

A targeted `eprintln!` is already in place in `set_vertical_offset` (in `src/ppu/mod.rs`):

```rust
if number == 2 {
    eprintln!("BG2VOFS write: value={:#04X} bg_old={:#04X} => offset={}", value, self.bg_old, offset);
}
```

**Next step**: run the lag test and examine the BG2VOFS write log. Look for:
- What value/bg_old combination produces offset=65535 (should be value=0xFF, bg_old=0xFF)
- Whether it happens once at startup (stale bg_old) or every other frame (HDMA writing bad data)
- Compare write patterns between a "good" frame (vofs=0) and a "bad" frame (vofs=65535)

## Run command

```
cargo run --release -- /Users/christopherk/Desktop/Files/240pTestSuite-SNES-1.09/240pSuite.sfc
```

Navigate to the Lag Test screen. Pipe stderr to a file to avoid terminal flood:
```
cargo run --release -- /path/to/240pSuite.sfc 2>bg2vofs.log
```
Then inspect `bg2vofs.log`.
