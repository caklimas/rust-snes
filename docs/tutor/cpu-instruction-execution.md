# How CPU Instructions Execute

## Prerequisites
- [CPU Overview](cpu-65c816-overview.md) — registers, flags, and modes

## The Fetch-Decode-Execute Cycle

Every instruction follows the same loop:

```
┌──────────────────────────────────────────────────┐
│                                                  │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐  │
└──▶│  FETCH   │───▶│  DECODE  │───▶│ EXECUTE  │──┘
    │          │    │          │    │          │
    │ Read the │    │ Figure   │    │ Do the   │
    │ next     │    │ out what │    │ thing    │
    │ byte(s)  │    │ it means │    │          │
    └──────────┘    └──────────┘    └──────────┘
```

1. **Fetch**: Read 1 byte from `PB:PC` — this is the opcode
2. **Decode**: The opcode byte tells the CPU what instruction and addressing mode
3. **Execute**: Fetch operands, compute addresses, perform the operation

## Concrete Example: `LDA #$42` (8-bit mode, M=1)

Memory at $00:8000:
```
$00:8000  $A9     ← opcode: LDA immediate
$00:8001  $42     ← the value to load
```

Cycle-by-cycle:
```
Cycle 1:  Read $A9 from $00:8000.  Decode: "LDA immediate"
          PC → $8001

Cycle 2:  Read $42 from $00:8001.  Store in A.
          Update Z flag (not zero), N flag (bit 7=0)
          PC → $8002
```
**2 cycles total.**

## Same Opcode, 16-bit mode (M=0)

```
$00:8000  $A9     ← same opcode!
$00:8001  $42     ← low byte
$00:8002  $00     ← high byte (extra byte!)
```

```
Cycle 1:  Read $A9.           PC → $8001
Cycle 2:  Read $42 (lo).      PC → $8002
Cycle 3:  Read $00 (hi).      A = $0042. Update flags.
          PC → $8003
```
**3 cycles — one extra for the extra byte.**

Same opcode, different instruction length. The M flag changes how many bytes the CPU reads.

## Complex Example: `LDA $1234,X` (Absolute Indexed)

Assume DB=$7E, X=$0010:
```
$00:8000  $BD     ← opcode: LDA absolute,X
$00:8001  $34     ← address low
$00:8002  $12     ← address high
```

```
Cycle 1:  Read opcode $BD.                  PC → $8001
Cycle 2:  Read $34 (addr lo).               PC → $8002
Cycle 3:  Read $12 (addr hi).               PC → $8003
Cycle 4:  Effective addr = DB:(operand+X)
          = $7E:($1234+$0010) = $7E:1244
          Read from $7E:1244 → store in A.
```
**4 cycles** (5 if index crosses a page boundary).

## The General Pattern

```
Phase              What happens                      Cycles
─────────────────────────────────────────────────────────────
1. Fetch opcode    Read 1 byte at PB:PC              1
2. Fetch operand   Read 0-3 more bytes               0-3
                   (depends on addressing mode)
3. Calculate       Compute effective address          0-1
   address         (extra cycle if page crossing)
4. Read/Modify/    Access memory at effective addr    1-2
   Write           (RMW instructions do both)
─────────────────────────────────────────────────────────────
Total                                                 2-8
```

More complex addressing = more memory reads = more cycles.

## Scanline Timing and Master Clocks

### Master Clock

The SNES master clock runs at **21.477 MHz** (NTSC). Everything derives from this.

Each CPU memory access takes a different number of master clocks depending on what address is being touched:

```
┌──────────────────────────────────────────────┬────────┐
│ Address Range                                │ Clocks │
├──────────────────────────────────────────────┼────────┤
│ Internal CPU operations (no bus access)      │   6    │
│ PPU/APU registers ($2100-$21FF)              │   6    │
│ CPU I/O registers ($4200-$5FFF)              │   6    │
│                                              │        │
│ WRAM mirror ($0000-$1FFF in banks $00-$3F)   │   8    │
│ WRAM ($7E:0000-$7F:FFFF)                     │   8    │
│ ROM banks $00-$3F:$8000-$FFFF  (SlowROM)     │   8    │
│ ROM banks $40-$7D:$0000-$FFFF  (HiROM)       │   8    │
│                                              │        │
│ ROM banks $80-$BF:$8000-$FFFF  (WS2)        │ 6 or 8 │
│ ROM banks $C0-$FF:$0000-$FFFF  (WS2)        │ 6 or 8 │
│  └─ 6 if MEMSEL ($420D) bit 0 = 1 (FastROM) │        │
│  └─ 8 if MEMSEL ($420D) bit 0 = 0 (default) │        │
│                                              │        │
│ Joypad serial ($4016-$4017)                  │  12    │
└──────────────────────────────────────────────┴────────┘
```

### Scanline Budget

```
Total master clocks per scanline:          1364
DRAM refresh (once per scanline):          - 40
Available for CPU:                         1324 master clocks
```

### Level 2 Approximation

Instead of tracking per-access speeds, approximate by the region the CPU is executing from:

```
master_clocks_consumed = cpu_cycles × speed_of(PB:PC region)
```

This captures the key difference: code in FastROM gets ~227 cycles/scanline, code in SlowROM gets ~165 cycles/scanline.

## CPU Stalls

The CPU halts during:
- **DMA transfers** — CPU completely stopped until transfer finishes
- **HDMA** — steals cycles at the start of each HBlank
- **WAI instruction** — CPU voluntarily sleeps until next interrupt

## Related Topics
- [Addressing Modes](cpu-addressing-modes.md) — the 24 ways to form effective addresses
- [Interrupts](cpu-interrupts.md) — NMI, IRQ, and how they break into the cycle
