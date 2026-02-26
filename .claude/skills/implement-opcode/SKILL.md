---
name: implement-opcode
description: Implement and fix 65C816 CPU opcodes for the SNES emulator. Use when implementing new opcodes, fixing opcode test failures, or debugging CPU instruction behavior.
argument-hint: "[opcode-hex-or-error-message]"
---

# Implement/Fix 65C816 Opcodes

## 65C816 Reference Documentation

Use these resources to look up opcode behavior, addressing modes, cycle counts, and flag effects:

- **65C816 opcode matrix & reference**: https://undisbeliever.net/snesdev/65816-opcodes.html
- **65C816 instruction set (detailed)**: https://www.westerndesigncenter.com/wdc/documentation/w65c816s.pdf (WDC official datasheet)
- **Fullsnes SNES doc**: https://problemkaputt.de/fullsnes.htm
- **SNESdev Wiki**: https://snes.nesdev.org/wiki/65C816_reference

## Codebase Architecture

### Opcode Modules
Each instruction family has its own file in `src/cpu/opcodes/`:
- `adc.rs`, `sbc.rs` - arithmetic (with BCD decimal mode support)
- `and.rs`, `ora.rs`, `eor.rs` - logic
- `lda.rs`, `ldx.rs`, `ldy.rs` - loads
- `sta.rs`, `stx.rs`, `sty.rs`, `stz.rs` - stores
- `cmp.rs`, `cpx.rs`, `cpy.rs` - comparisons
- `shift.rs` - ASL, LSR, ROL, ROR
- `inc.rs`, `dec.rs` - increment/decrement
- `bra.rs` - branches
- `jmp.rs`, `jsr.rs`, `ret.rs` - jumps/calls/returns
- `stack.rs` - push/pull/stack ops
- `transfer.rs` - register transfers
- `flags.rs` - flag manipulation (SEC, CLC, SEP, REP, etc.)
- `bit.rs` - BIT, TSB, TRB
- `block_move.rs` - MVN, MVP
- `misc.rs` - NOP, XBA, XCE, WAI, STP, WDM

### Opcode Dispatcher
`src/cpu/opcodes/mod.rs` contains `execute_opcode()` which maps opcode bytes to handler functions. Add new opcodes here.

### Helper Functions (IMPORTANT - use these!)
All shared logic lives in `src/cpu/opcodes/helpers/`:

#### `helpers/addressing.rs` - Address calculation
- `calculate_direct_page_address(cpu, bus)` - Direct page address (bank 0)
- `calculate_direct_page_x_address(cpu, bus)` - Direct page + X
- `calculate_direct_page_y_address(cpu, bus)` - Direct page + Y
- `calculate_indirect_page_address(cpu, bus)` - (dp) indirect
- `calculate_indirect_page_x_address(cpu, bus)` - (dp,X) indirect
- `calculate_indirect_page_y_address(cpu, bus)` - (dp),Y indirect
- `calculate_absolute_x_address(cpu, bus)` - Absolute + X (16-bit)
- `calculate_absolute_y_address(cpu, bus)` - Absolute + Y (16-bit)
- `calculate_absolute_long_address(cpu, bus)` - 24-bit absolute long
- `calculate_absolute_long_x_address(cpu, bus)` - 24-bit absolute long + X
- `calculate_stack_relative_address(cpu, bus)` - Stack relative (sr,S)
- `increment_program_counter(cpu, n)` - Advance PC by n bytes
- `get_x_register_value(cpu)` / `get_y_register_value(cpu)` - Get X/Y respecting 8/16-bit mode
- `page_crossed(base, effective)` - Check if addresses cross a page boundary

#### `helpers/memory.rs` - Memory access
**CRITICAL: Use the correct bank for each addressing mode!**

| Function | Bank | Use for |
|----------|------|---------|
| `read_offset_byte(cpu, bus)` | PBR:PC+1 | Reading instruction operands |
| `read_offset_word(cpu, bus)` | PBR:PC+1..+2 | Reading instruction operands |
| `read_program_byte(cpu, bus, addr)` | PBR:addr | Instruction fetches, JMP (abs,X) pointer reads |
| `read_program_word(cpu, bus, addr)` | PBR:addr | Instruction fetches, JMP (abs,X) pointer reads |
| `read_data_byte(cpu, bus, addr)` | DBR:addr | **Absolute addressing data reads** |
| `read_data_word(cpu, bus, addr)` | DBR:addr | **Absolute addressing data reads** |
| `read_byte_direct_page(bus, addr)` | 00:addr | **Direct page reads** (always bank 0) |
| `read_word_direct_page(bus, addr)` | 00:addr | **Direct page reads** (always bank 0) |
| `read_word_direct_page_wrapped(cpu, bus, addr)` | 00:addr | DP 16-bit read with page-wrap (use for PEI and any instruction that reads a 16-bit value from DP in emulation mode) |
| `read_long_pointer_direct_page(bus, addr)` | 00:addr | Read 3-byte (24-bit) pointer from DP |
| `write_data_byte(cpu, bus, addr, val)` | DBR:addr | **Absolute addressing data writes** |
| `write_data_word(cpu, bus, addr, val)` | DBR:addr | **Absolute addressing data writes** |
| `write_byte_direct_page(bus, addr, val)` | 00:addr | **Direct page writes** (always bank 0) |
| `write_word_direct_page(bus, addr, val)` | 00:addr | **Direct page writes** (always bank 0) |

#### `helpers/flags.rs` - Flag operations
- `is_8bit_mode_m(cpu)` - Check if M flag set (8-bit accumulator/memory)
- `is_8bit_mode_x(cpu)` - Check if X flag set (8-bit index registers)
- `set_nz_flags_u8(cpu, value)` - Set N and Z flags for 8-bit result
- `set_nz_flags_u16(cpu, value)` - Set N and Z flags for 16-bit result
- `get_carry_in(cpu)` - Get carry flag as u16 (0 or 1)

#### `helpers/stack.rs` - Stack operations
- `push_byte(cpu, bus, value)` / `pull_byte(cpu, bus)` - 8-bit stack ops
- `push_word(cpu, bus, value)` / `pull_word(cpu, bus)` - 16-bit stack ops

## Common Bug Classes (watch for these!)

### 1. Wrong bank for memory access
- **Direct page** operations MUST use bank 0 (`read_byte_direct_page`, `write_byte_direct_page`)
- **Absolute** data reads/writes MUST use DBR (`read_data_byte`, `write_data_byte`)
- **Indirect** data reads MUST use DBR (`read_data_byte`, `read_data_word`)
- **JMP indirect** (0x6C, 0xDC) reads pointer from bank 0, NOT PBR
- **JMP (abs,X)** (0x7C) reads pointer from PBR (this one IS correct with read_program_word)
- Only instruction fetches and operand reads use PBR

### 2. Wrong write helper for direct page stores
Store instructions (STA, STX, STY, STZ) writing to **direct page** must use `write_byte_direct_page(bus, addr, val)` / `write_word_direct_page(bus, addr, val)` (bank 0), NOT `write_data_byte`/`write_data_word` (which apply DBR and write to the wrong bank when DBR ≠ 0).

### 3. PC wrapping in operand reads
Use `cpu.registers.pc.wrapping_add(n)` instead of `cpu.registers.pc + n` when reading operand bytes via `read_program_byte`. Plain `+` panics on overflow when PC is near `0xFFFF`.

### 4. Missing direct page cycle penalty
Any instruction using direct page addressing needs `+1 cycle when D register low byte != 0`:
```rust
if (cpu.registers.d & 0x00FF) != 0 {
    cycles += 1;
}
```

### 5. Wrong cycle counts for 16-bit read-modify-write
Read-modify-write instructions (ASL, LSR, ROL, ROR, INC, DEC) on direct page add **+2 cycles** for 16-bit mode (M=0), not +1. Same applies to absolute addressing. Reference: `asl_direct` (5 8-bit / 7 16-bit), `dec_absolute` (6 8-bit / 8 16-bit).

### 6. BCD (Decimal mode) in ADC/SBC
ADC and SBC must handle decimal mode (D flag set). Key points:
- Process each nibble (4 bits) separately, applying +6 or -6 correction
- V flag is computed from the INTERMEDIATE result (after low nibble correction, before high nibble correction)
- N and Z flags are from the FINAL BCD-corrected result
- C flag is from the BCD computation
- SBC in decimal mode is implemented as ADC with one's-complemented operand: `A + ~M + C`
- 16-bit BCD requires 4-nibble processing

### 5. Branch page crossing penalty (emulation mode only)
Branch instructions (BRA, BEQ, BNE, BCC, BCS, BMI, BPL, BVC, BVS) add +1 cycle when crossing a page boundary **only in emulation mode**. In native mode there is no page crossing penalty for branches:
```rust
if cpu.emulation_mode && page_crossed { cycles += 1; }
```

### 7. REP in emulation mode
In emulation mode, REP cannot clear the M (0x20) and X (0x10) flags — they are hardwired to 1. After `cpu.registers.p.remove(bits_to_clear)`, re-insert them if `cpu.emulation_mode`:
```rust
if cpu.emulation_mode {
    cpu.registers.p.insert(ProcessorStatus::MEMORY_WIDTH | ProcessorStatus::INDEX_WIDTH);
}
```

### 8. Direct page 16-bit read page-wrapping
In emulation mode with DL=0, when reading a 16-bit value from a DP address at `0xXXFF`, the second byte wraps to `0xXX00` (stays within the 256-byte page). Use `read_word_direct_page_wrapped(cpu, bus, addr)` instead of `read_word_direct_page` for instructions that read 16-bit values from DP in emulation mode (e.g., PEI).

### 9. Page crossing penalty (absolute indexed)
Absolute,X and Absolute,Y addressing modes add +1 cycle when the indexing crosses a page boundary:
```rust
if page_crossed(base_address, effective_address) {
    cycles += 1;
}
```

## Workflow Rules (IMPORTANT)

### When fixing an existing (already-implemented) opcode
1. Fix it — no need to ask permission first
2. Run the tests
3. **STOP and explain what was fixed and why**
4. **Wait for the user to say "proceed" before continuing**

### When implementing a new (unimplemented) opcode
- Implement, run tests, and continue without pausing unless a test failure reveals a bug in an existing opcode

### files_to_skip
- **NEVER add entries to `files_to_skip` in `tests/integration_test.rs` without first discussing it with the user**
- If a test file causes a JSON parse error or other infrastructure failure, explain the issue and ask what to do — don't silently skip it
- The existing skipped files (44.e, 44.n, 54.e, 54.n) are for a known reason that may be investigated later

### skip_count
- Update `skip_count` in `tests/integration_test.rs` to point at the current failing opcode each time, to avoid re-running already-passing tests
- To find the correct value: `ls external/ProcessorTests/65816/v1/ | sort | grep -n "^<hex>\."` gives the 1-indexed position; subtract 1 for skip_count

## How to Implement a New Opcode

1. **Look up the opcode** in the 65816 reference to understand its addressing mode, affected flags, and cycle counts
2. **Find a similar existing opcode** in the same instruction family or with the same addressing mode to use as a template
3. **Add the function** in the appropriate `src/cpu/opcodes/<family>.rs` file
4. **Wire it up** in `src/cpu/opcodes/mod.rs` in the `execute_opcode()` match
5. **Run integration tests** to verify:
   ```bash
   cargo test --test integration_test -- --nocapture
   ```

## Testing Infrastructure

### Test files
Tests are in `external/ProcessorTests/65816/v1/` as JSON files named `<hex>.e.json` (emulation mode) and `<hex>.n.json` (native mode). Each file contains 10,000 test cases with initial/final CPU state and expected cycle counts.

### Test runner
`tests/common/test_runner.rs` validates: PC, S, A, X, Y, P (flags), DB, D, PB, E (emulation mode), memory contents, and cycle counts.

### Integration test
`tests/integration_test.rs` iterates through test files. It has a `skip_count` variable to resume from a specific opcode and a `files_to_skip` list for known-skipped opcodes (currently 44.e, 44.n, 54.e, 54.n for MVN/MVP).

### Error format
Test failures look like: `[6d e 1] A mismatch: expected 19698, got 19562`
- `6d` = opcode hex
- `e`/`n` = emulation/native mode
- `1` = test case number
- Then the field and values

## Currently Unimplemented Opcodes

**The authoritative source is `src/cpu/opcodes/mod.rs`** — check the `execute_opcode()` match for gaps (opcodes missing from the match fall through to the `_ => unimplemented` arm). The table below reflects the current state but may be stale; always verify against `mod.rs`.

| Opcode | Instruction | Addressing Mode |
|--------|------------|-----------------|
| 0xDF | CMP | Absolute Long X |
| 0xE3 | SBC | Stack Relative |
| 0xE7 | SBC | Direct Page Indirect Long |
| 0xEF | SBC | Absolute Long |
| 0xF3 | SBC | Stack Relative Indirect Indexed Y |
| 0xF7 | SBC | Direct Page Indirect Long Y |
| 0xFC | JSR | Absolute Indexed Indirect |
| 0xFF | SBC | Absolute Long X |

When implementing, look at existing implementations of the same instruction (e.g., for ADC long, look at `adc_absolute`, `adc_indirect_long`, and how ORA/AND/EOR handle their long addressing modes).
