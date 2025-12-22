// Integration tests for the common test infrastructure
mod common;

use common::test_bus::TestBus;
use common::test_format::{CpuState, TestCase};
use common::test_helpers::{extract_cpu_state, setup_cpu_from_state};
use rust_snes::cpu::Cpu;

#[test]
fn test_bus_basic_operations() {
    let mut bus = TestBus::new();

    // Uninitialized memory returns 0
    assert_eq!(bus.read(0x1000), 0);

    // Write and read back
    bus.write(0x1000, 0x42);
    assert_eq!(bus.read(0x1000), 0x42);
}

#[test]
fn test_deserialize_test_case() {
    let json = r#"
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
    "#;

    let test: TestCase = serde_json::from_str(json).unwrap();

    assert_eq!(test.name, "ad 00 20");
    assert_eq!(test.initial.pc, 4096);
    assert_eq!(test.initial.a, 0);
    assert_eq!(test.initial.db, 0);
    assert_eq!(test.initial.pb, 0);
    assert_eq!(test.cycles.len(), 4);
}

#[test]
fn test_cpu_state_setup() {
    let initial_state = CpuState {
        pc: 0x1000,
        s: 0x01FF,
        a: 0x1234,
        x: 0x5678,
        y: 0x9ABC,
        p: 0b11001010,
        db: 0x12,
        d: 0x2000,
        pb: 0x34,
        e: 0,
        ram: vec![],
    };

    let mut cpu = Cpu::default();
    setup_cpu_from_state(&mut cpu, &initial_state);

    assert_eq!(cpu.registers.pc, 0x1000);
    assert_eq!(cpu.registers.s, 0x01FF);
    assert_eq!(cpu.registers.a, 0x1234);
    assert_eq!(cpu.registers.db, 0x12);
    assert_eq!(cpu.registers.pb, 0x34);
    assert!(!cpu.emulation_mode);

    let extracted = extract_cpu_state(&cpu);
    assert_eq!(extracted.pc, initial_state.pc);
    assert_eq!(extracted.a, initial_state.a);
}
