# SNES Cartridge Reading

## The Problem

A SNES ROM file is just a flat array of bytes on disk. The CPU accesses memory using
24-bit addresses (`$00:0000` – `$FF:FFFF` = 16MB address space). The cartridge only
occupies part of that space, and how it maps depends on the **ROM type**.

---

## Step 1: The SMC Header (Gotcha — Check This First)

Many `.smc` ROM files have a **512-byte copier header** prepended before the actual ROM data.

**Detection:** `file_size % 1024 == 512`

If true, skip the first 512 bytes before doing anything else. All offsets below assume
this has already been stripped.

---

## Step 2: The ROM Header

Every SNES ROM has a header embedded at a fixed location inside the file. It contains:

- Game title
- ROM size
- SRAM size
- **Mapping mode** (the key byte)

### Mapping Mode Byte

Located at header offset `$15` (absolute address `$FFD5` in LoROM space):

| Value  | Mapping Mode        |
|--------|---------------------|
| `0x20` | LoROM               |
| `0x21` | HiROM               |
| `0x23` | SA-1 (ignore for now) |
| `0x25` | ExHiROM (large games) |

### The Bootstrap Problem

You don't know the mapping mode until you've read the header, but you need the mapping
mode to know where the header is. The solution:

1. Check the LoROM header candidate location
2. Check the HiROM header candidate location
3. Score each by validating the checksum and checking for sensible values
4. Whichever scores higher wins

### Header Candidate Locations (in the flat ROM file)

| Mode   | File Offset of Header Start |
|--------|-----------------------------|
| LoROM  | `$007FB0` (or `$007FC0` for title) |
| HiROM  | `$00FFB0` (or `$00FFC0` for title) |

### Checksum Validation

The header contains:
- `$FFDe–$FFDf`: Checksum complement (`~checksum`)
- `$FFDc–$FFDd`: Checksum (sum of all ROM bytes, lower 16 bits)

A valid header satisfies: `checksum + complement == 0xFFFF`

---

## Step 3: Address Mapping

Once you know the mapping mode, you can translate any 24-bit SNES address into a flat
ROM file offset.

### LoROM

Each 32KB bank of ROM maps to `$8000–$FFFF` of a SNES bank.

```
SNES $00:8000–$00:FFFF  →  ROM offset $000000–$007FFF  (bank 0)
SNES $01:8000–$01:FFFF  →  ROM offset $008000–$00FFFF  (bank 1)
SNES $02:8000–$02:FFFF  →  ROM offset $010000–$017FFF  (bank 2)
...
```

**Formula:**
```
bank       = (snes_address >> 16) & 0x7F   // mask mirrors ($80–$FF mirror $00–$7F)
offset     = snes_address & 0x7FFF         // lower 15 bits (within the 32KB window)
file_offset = (bank * 0x8000) + offset
```

### HiROM

Each 64KB bank of ROM maps to the full `$0000–$FFFF` of a SNES bank (starting at bank `$40`).

```
SNES $40:0000–$40:FFFF  →  ROM offset $000000–$00FFFF  (bank 0)
SNES $41:0000–$41:FFFF  →  ROM offset $010000–$01FFFF  (bank 1)
...
```

**Formula:**
```
bank        = (snes_address >> 16) & 0x3F  // mask mirrors
offset      = snes_address & 0xFFFF
file_offset = (bank * 0x10000) + offset
```

---

## Step 4: The Reset Vector

Once the cartridge is mapped, the reset vector is just two bytes in ROM:

| Mode          | Address       |
|---------------|---------------|
| Native mode   | `$00:FFFC–$00:FFFD` |
| Emulation mode | `$00:FFFC–$00:FFFD` (same location, bank 0) |

Read these two bytes through the mapped cartridge, combine them into a `u16`, and set
`cpu.registers.pc` to that value on startup.

---

## Summary: What Needs to Be Written

| Component | Responsibility |
|---|---|
| `Cartridge::new()` | Strip SMC header, detect LoROM/HiROM via header scoring, store mapping mode |
| `Cartridge::read(address: u32)` | Translate 24-bit SNES address → flat file offset, return byte |
| `Bus::read()` | Call `self.cartridge.read(address)` instead of returning `0` for the cartridge ROM range |
| `SuperNintendo::new()` | After bus is created, read `$FFFC–$FFFD` and set `cpu.registers.pc` |

The address translation in `Cartridge::read()` is the meaty part. Everything else
is fairly mechanical.
