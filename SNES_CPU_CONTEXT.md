# SNES / 65816 Emulator – Context Handoff

This project is a Rust-based SNES emulator.  
Recent work focused on **CPU opcode correctness**, especially tricky **65816 edge cases** involving:

- stack-relative addressing
- direct-page (DP) bank behavior
- direct-page timing penalties
- read-modify-write (RMW) cycle counts
- TSB / TRB correctness

The goal when resuming is to **fix existing opcodes for correctness** before adding new ones.

---

## Core Opcode Infrastructure

### Opcode helpers, dispatch, addressing helpers

**Raw file:**
https://raw.githubusercontent.com/caklimas/rust-snes/opcode-testing/src/cpu/opcodes/mod.rs

Contains:

- opcode dispatch
- addressing helpers
- accumulator/index width helpers
- program counter helpers
- MemoryBus abstractions

---

## BIT / TSB / TRB Opcode Implementation

### BIT / TSB / TRB opcodes

**Raw file:**
https://raw.githubusercontent.com/caklimas/rust-snes/opcode-testing/src/cpu/opcodes/bit.rs

Focus:

- tsb_direct correctness
- bank-0 DP writes
- RMW semantics
- correct cycle counts

---

## Key Fixes Applied

### 1. Direct Page Writes Must Use Bank 0

Direct page reads/writes must occur in **bank 0**.
Do not use DBR-based helpers for DP writes.

### 2. Direct Page Timing Penalty

Add **+1 cycle** when:
(cpu.registers.d & 0x00FF) != 0

### 3. Read-Modify-Write Cycle Rule

Base cycles for DP RMW instructions:

- 8-bit: 5
- 16-bit: 7
  Add DP penalty if applicable.

### 4. Stack-Relative Addressing

- (sr,S) uses bank 0
- (sr,S),Y reads pointer from bank 0, final access uses DBR

---

## Opcode Recently Fixed

- 0x04 — TSB direct

---

## Recommended Next Steps

- Audit all DP RMW opcodes
- Ensure TRB mirrors TSB
- Continue correctness fixes before adding new opcodes
