# The 65C816 CPU — SNES Brain

## Prerequisites
- [System Overview](system-overview.md) — understand where the CPU fits in the system

## What Is It?

The SNES CPU is a **WDC 65C816**, packaged by Ricoh as the **5A22** (which adds DMA hardware, multiplication/division registers, and interrupt controllers around the base CPU). It's an evolution of the **6502** — the 8-bit CPU from the NES, Apple II, and Commodore 64.

Nintendo chose it for backward compatibility with the 6502 instruction set and because it extends to 16-bit registers and a 24-bit address bus (16 MB address space).

## Registers

```
┌─────────────────────────────────────────────────────┐
│                   65C816 Registers                   │
├──────────┬──────┬───────────────────────────────────┤
│ Register │ Size │ Purpose                           │
├──────────┼──────┼───────────────────────────────────┤
│ A        │ 8/16 │ Accumulator — math & logic ops    │
│ X        │ 8/16 │ Index register — array indexing   │
│ Y        │ 8/16 │ Index register — array indexing   │
│ S        │ 16   │ Stack Pointer                     │
│ PC       │ 16   │ Program Counter (within bank)     │
│ P        │ 8    │ Processor Status (flags)          │
│ D        │ 16   │ Direct Page register              │
│ DB       │ 8    │ Data Bank register                │
│ PB       │ 8    │ Program Bank register             │
└──────────┴──────┴───────────────────────────────────┘
```

### Accumulator (A)
Where most computation happens — addition, subtraction, logic, comparisons all flow through A. Can be 8-bit or 16-bit depending on the M flag.

When 16-bit, the low byte is called A (or AL) and the high byte is B (or AH). Together: C (the full 16-bit accumulator).

### Index Registers (X, Y)
Used to index into arrays/tables. Width controlled by the X flag, independently from A.

### Address Formation: PC, PB, DB

The 6502 had a 16-bit address space (64 KB). The 65C816 extends to 24-bit (16 MB) using bank registers:

```
  Full 24-bit address = Bank : Offset
                        (8)    (16)

  For code:   PB : PC    →  "Where am I executing?"
  For data:   DB : addr  →  "Where am I reading/writing?"
```

- **PB (Program Bank)** — bank the CPU fetches instructions from
- **DB (Data Bank)** — default bank for data reads/writes
- **PC (Program Counter)** — next instruction within the current bank

Example: PB=$80, PC=$8000 → CPU fetches from $80:8000.

### Direct Page Register (D)
Generalizes the 6502's "zero page" ($00-$FF) — the fast-access memory window can be relocated anywhere in the first 64 KB by setting D. If D=$1100, direct page offset $20 → address $1120.

### Stack Pointer (S)
16-bit in native mode. The stack can live anywhere in bank 0 (not just page $01 like the 6502).

## Processor Status Register (P)

```
  Bit:  7   6   5   4   3   2   1   0
        N   V   M   X   D   I   Z   C
        │   │   │   │   │   │   │   │
        │   │   │   │   │   │   │   └── Carry
        │   │   │   │   │   │   └────── Zero (result was 0)
        │   │   │   │   │   └────────── IRQ Disable
        │   │   │   │   └────────────── Decimal Mode (BCD)
        │   │   │   └────────────────── Index width (1=8-bit, 0=16-bit)
        │   │   └────────────────────── Accumulator width (1=8-bit, 0=16-bit)
        │   └────────────────────────── Overflow
        └────────────────────────────── Negative (sign bit)
```

### Condition Flags (set automatically)
| Flag | Meaning |
|------|---------|
| **N** | Negative — high bit of result is 1 |
| **V** | Overflow — signed arithmetic overflowed |
| **Z** | Zero — result was exactly 0 |
| **C** | Carry — unsigned arithmetic carried / didn't borrow |

### Control Flags (set deliberately)
| Flag | Meaning |
|------|---------|
| **M** | 1 = A is 8-bit, 0 = A is 16-bit |
| **X** | 1 = X/Y are 8-bit, 0 = X/Y are 16-bit |
| **D** | Decimal mode (BCD arithmetic, rarely used on SNES) |
| **I** | 1 = mask IRQ interrupts |

## Emulation Mode vs Native Mode

The 65C816 boots in **emulation mode** — behaves like a 6502:
- A, X, Y locked to 8-bit
- Stack locked to page $01 ($0100-$01FF)
- No bank registers (everything in bank 0)
- M and X flag bits have different meanings (6502 B flag and unused)

Switch to **native mode** with:
```asm
CLC        ; Clear carry
XCE        ; Exchange Carry ↔ Emulation flag
```

The **E (emulation) flag** is hidden — not in P register, only accessible by swapping with Carry via XCE. Once in native mode, the full 65C816 features are available.

**Every SNES game switches to native mode almost immediately on boot.**

## The Width-Switching Dance

`SEP` and `REP` instructions change M and X flags:

```asm
REP #$30    ; Clear bits 5,4 → M=0, X=0 → A=16-bit, X/Y=16-bit
SEP #$20    ; Set bit 5      → M=1       → A=8-bit
REP #$10    ; Clear bit 4    → X=0       → X/Y=16-bit
```

**The same opcode can be different sizes depending on flags!** For example, `LDA #imm`:
- M=0 (16-bit A): 3-byte instruction (opcode + 2 bytes)
- M=1 (8-bit A): 2-byte instruction (opcode + 1 byte)

This makes disassembly require tracking flag state at all times.

## The 24-Bit Address Space

```
$00:0000 ┬──────────────────────┐
         │  Bank $00            │  64 KB
$00:FFFF ┴──────────────────────┘
$01:0000 ┬──────────────────────┐
         │  Bank $01            │  64 KB
$01:FFFF ┴──────────────────────┘
         │       ...            │
$7E:0000 ┬──────────────────────┐
         │  WRAM (128 KB)       │
$7F:FFFF ┴──────────────────────┘
         │       ...            │
$FF:FFFF └──────────────────────┘
           256 banks × 64 KB = 16 MB
```

Different banks map to ROM, WRAM, or I/O depending on the cartridge mapping mode (LoROM/HiROM).

## What Makes the 65C816 Special

1. **Variable-width registers** — A, X, Y can be 8 or 16-bit at runtime
2. **24-bit addressing** via bank registers — 16 MB from 8-bit heritage
3. **Relocatable Direct Page** — fast-access window can move anywhere
4. **Two modes** — emulation (6502 compat) and native (full power)
5. **Same instruction set core as 6502** — plus ~78 new opcodes

## Related Topics
- [Addressing Modes](cpu-addressing-modes.md) — the 24 ways to specify operands
- [Interrupts & Vectors](cpu-interrupts.md) — NMI, IRQ, and exception handling
- [Memory Map](memory-map.md) — how the 24-bit address space is laid out
