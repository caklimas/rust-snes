# CPU Testing Guide

This guide explains how to test the 65816 CPU implementation using the [SingleStepTests/65816](https://github.com/SingleStepTests/65816) test suite.

## Test Suite Overview

**Test Coverage:**
- **20,000 tests per opcode**
  - 10,000 tests in **native mode** (16-bit 65816 mode)
  - 10,000 tests in **emulation mode** (6502 compatibility mode)
- Tests for all 65816 opcodes
- JSON-encoded format (language-agnostic)

**Test Structure:**
Each test includes:
1. `name`: Human-readable identifier
2. `initial`: Starting CPU and memory state (registers, flags, memory contents)
3. `final`: Expected CPU and memory state after execution
4. `cycles`: **Cycle-by-cycle bus activity breakdown**

**Key Points:**
- Assumes 16MB of RAM available (full 65816 address space)
- Single address space
- Each test focuses on **one instruction**
- Provides detailed bus activity per cycle

## Test File Structure

Each JSON test file contains an array of test cases. Each test looks something like this:

```json
{
  "name": "ad 00 20",
  "initial": {
    "pc": 4096,
    "s": 253,
    "a": 0,
    "x": 0,
    "y": 0,
    "p": 48,
    "dbr": 0,
    "d": 0,
    "pbr": 0,
    "e": 0,
    "ram": [
      [4096, 173],
      [4097, 0],
      [4098, 32],
      [8192, 66]
    ]
  },
  "final": {
    "pc": 4099,
    "s": 253,
    "a": 66,
    "x": 0,
    "y": 0,
    "p": 48,
    "dbr": 0,
    "d": 0,
    "pbr": 0,
    "e": 0,
    "ram": [
      [4096, 173],
      [4097, 0],
      [4098, 32],
      [8192, 66]
    ]
  },
  "cycles": [
    [4096, 173, "read"],
    [4097, 0, "read"],
    [4098, 32, "read"],
    [8192, 66, "read"]
  ]
}
```

## Understanding the Fields

### Processor State Fields:
- `pc`: Program counter (16-bit)
- `s`: Stack pointer (16-bit)
- `a`: Accumulator (16-bit, but might only use lower 8 bits in 8-bit mode)
- `x`: X register (16-bit)
- `y`: Y register (16-bit)
- `p`: Processor status flags (8-bit)
- `dbr`: Data bank register (8-bit)
- `d`: Direct page register (16-bit)
- `pbr`: Program bank register (8-bit)
- `e`: Emulation mode flag (0 = native, 1 = emulation)

### Memory Format:
- `ram`: Array of `[address, value]` pairs
- Only lists memory locations that are **non-zero** or **relevant** to the test
- Address is the **full 24-bit address** (not segmented)

### Cycles Array:
- Each entry: `[address, value, operation]`
- `operation`: "read" or "write"
- Shows **every bus transaction** cycle-by-cycle
- Length of array = total cycles the instruction took

## Test Execution Workflow

Here's the conceptual flow for running one test:

### 1. Setup Phase
```
- Clear all memory (or set to known state)
- Load initial RAM values from test.initial.ram
- Set all CPU registers from test.initial
- Set emulation mode flag
```

### 2. Execution Phase
```
- Call cpu.step() to execute one instruction
- Capture the number of cycles returned
```

### 3. Verification Phase
```
- Compare CPU registers against test.final
  - pc, s, a, x, y, p, dbr, d, pbr, e all match?
- Compare RAM state against test.final.ram
  - For each address in test.final.ram, verify memory matches
- Compare cycle count
  - cycles_returned == test.cycles.length?
```

### 4. Report Results
```
- If all match: Test PASSED ✓
- If mismatch: Test FAILED ✗
  - Report which field(s) differed
  - Show expected vs actual
```

## Implementation Architecture

Here's what you'd need to build:

### 1. Test Loader
- Parse JSON files
- Deserialize into Rust structs
- Handle multiple test files (one per opcode)

### 2. CPU State Manager
- Function to **set up** CPU from initial state
- Function to **extract** CPU state for comparison
- Handle the mapping between test format and your CPU struct

### 3. Memory Manager
- Create a test bus/memory that can:
  - Be initialized from `ram` array
  - Be queried for verification
  - Support full 24-bit addressing (16MB)
- Reset between tests

### 4. Test Runner
- Load test file
- For each test:
  - Setup
  - Execute
  - Verify
  - Collect results
- Report summary (X/Y tests passed)

### 5. Comparison/Assertion Logic
- Compare register values
- Compare memory values
- Pretty-print differences when tests fail

## Key Challenges

### Challenge 1: Processor Flags
The `p` field is a single byte representing all flags. You'll need to:
- Convert between your `ProcessorStatus` bitflags and the u8 in tests
- Make sure flag bit positions match the 65816 spec

### Challenge 2: 16-bit vs 8-bit Modes
Some registers can be 8-bit or 16-bit depending on flags:
- Accumulator: Check M flag
- X/Y: Check X flag
- Tests will provide 16-bit values, but you need to mask appropriately

### Challenge 3: Memory Representation
The tests use sparse arrays `[[addr, val], [addr, val]]`:
- You'll need to translate this to/from your memory bus
- Only verify addresses that appear in `final.ram`
- Don't fail if other memory differs (tests only specify relevant locations)

### Challenge 4: 24-bit Addressing
Tests use flat 24-bit addresses:
- You need to convert to/from bank:offset if your emulator uses segmented addressing
- Example: Address `0x018000` = bank `0x01`, offset `0x8000`

### Challenge 5: Initial PC Handling
- The test's PC points to the opcode to execute
- After setup, your first `bus.read(pc)` should fetch the opcode
- Make sure you don't accidentally advance PC during setup

## Suggested Implementation Order

1. **Start small**: Pick one opcode file (like NOP or a simple LDA)
2. **Build the deserializer**: Get JSON → Rust structs working
3. **Implement setup**: Write code to configure CPU from initial state
4. **Run one test**: Execute and see what happens
5. **Add verification**: Compare results
6. **Handle failures**: Pretty-print what went wrong
7. **Scale up**: Run all tests in a file, then all files

## Example Pseudo-Code Structure

```rust
struct TestCase {
    name: String,
    initial: CpuState,
    final_state: CpuState,
    cycles: Vec<BusCycle>,
}

struct CpuState {
    pc: u16,
    s: u16,
    a: u16,
    x: u16,
    y: u16,
    p: u8,
    dbr: u8,
    d: u16,
    pbr: u8,
    e: u8,
    ram: Vec<(u32, u8)>,
}

fn run_test(test: &TestCase) -> TestResult {
    // 1. Create fresh CPU and bus
    let mut cpu = Cpu::default();
    let mut bus = TestBus::new();

    // 2. Apply initial state
    setup_cpu(&mut cpu, &test.initial);
    setup_memory(&mut bus, &test.initial.ram);

    // 3. Execute
    let cycles = cpu.step(&mut bus);

    // 4. Extract final state
    let actual_state = extract_cpu_state(&cpu);
    let actual_ram = extract_memory(&bus, &test.final_state.ram);

    // 5. Compare
    let passed =
        actual_state == test.final_state &&
        actual_ram == test.final_state.ram &&
        cycles == test.cycles.len();

    TestResult { passed, details: ... }
}
```

## Tips

- **Use serde_json** for parsing JSON
- **Create helper functions** for state setup/extraction
- **Start with emulation mode tests** (simpler, 8-bit only)
- **Log everything** when tests fail - you'll need to debug
- **Don't worry about cycle-by-cycle** bus activity initially - just verify total count
- **Expect failures** - this is how you'll find bugs in your opcodes!

## For Instruction-Level Accuracy

Since this emulator implements **instruction-level accuracy** (not cycle-level):
1. Use the `initial` state to set up your CPU/memory
2. Execute the instruction with `step()`
3. Compare against `final` state
4. Verify total cycle count matches the length of the `cycles` array
5. **(Optional)** Ignore the detailed cycle-by-cycle bus activity for now

The length of the `cycles` array tells you how many total cycles the instruction should take, which matches what your `step()` function returns.
