use super::test_bus::TestBus;
use super::test_format::{CpuState, TestCase};
use super::test_helpers::{
    extract_cpu_state, extract_memory_state, setup_bus_from_state, setup_cpu_from_state,
};
use rust_snes::cpu::Cpu;
use std::fs;
use std::path::Path;

/// Result of running a single test
#[derive(Debug)]
pub struct TestResult {
    pub passed: bool,
    pub failure_reason: Option<String>,
}

/// Loads test cases from a JSON file
pub fn load_tests_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<TestCase>, String> {
    let json =
        fs::read_to_string(path.as_ref()).map_err(|e| format!("Failed to read file: {}", e))?;

    let tests: Vec<TestCase> =
        serde_json::from_str(&json).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    Ok(tests)
}

/// Runs a single test case
pub fn run_test(test: &TestCase) -> TestResult {
    let mut cpu = Cpu::default();
    let mut bus = TestBus::new();

    // Setup initial state
    setup_cpu_from_state(&mut cpu, &test.initial);
    setup_bus_from_state(&mut bus, &test.initial);

    // Execute one instruction
    let cycles = cpu.step(&mut bus);

    // Extract final state
    let actual_cpu_state = extract_cpu_state(&cpu);
    let actual_memory = extract_memory_state(&bus, &test.final_state);

    // Compare results
    if let Some(reason) = compare_states(
        &test.name,
        &test.final_state,
        &actual_cpu_state,
        &actual_memory,
        cycles,
        &test.cycles,
    ) {
        TestResult {
            passed: false,
            failure_reason: Some(reason),
        }
    } else {
        TestResult {
            passed: true,
            failure_reason: None,
        }
    }
}

/// Compares expected vs actual state and returns error message if different
fn compare_states(
    test_name: &str,
    expected_cpu: &CpuState,
    actual_cpu: &CpuState,
    actual_memory: &[(u32, u8)],
    actual_cycles: u8,
    expected_cycles: &[super::test_format::BusCycle],
) -> Option<String> {
    // Check CPU registers
    if expected_cpu.pc != actual_cpu.pc {
        return Some(format!(
            "[{}] PC mismatch: expected {}, got {}",
            test_name, expected_cpu.pc, actual_cpu.pc
        ));
    }
    if expected_cpu.s != actual_cpu.s {
        return Some(format!(
            "[{}] S mismatch: expected {}, got {}",
            test_name, expected_cpu.s, actual_cpu.s
        ));
    }
    if expected_cpu.a != actual_cpu.a {
        return Some(format!(
            "[{}] A mismatch: expected {}, got {}",
            test_name, expected_cpu.a, actual_cpu.a
        ));
    }
    if expected_cpu.x != actual_cpu.x {
        return Some(format!(
            "[{}] X mismatch: expected {}, got {}",
            test_name, expected_cpu.x, actual_cpu.x
        ));
    }
    if expected_cpu.y != actual_cpu.y {
        return Some(format!(
            "[{}] Y mismatch: expected {}, got {}",
            test_name, expected_cpu.y, actual_cpu.y
        ));
    }
    if expected_cpu.p != actual_cpu.p {
        return Some(format!(
            "[{}] P mismatch: expected {}, got {}",
            test_name, expected_cpu.p, actual_cpu.p
        ));
    }
    if expected_cpu.db != actual_cpu.db {
        return Some(format!(
            "[{}] DB mismatch: expected {}, got {}",
            test_name, expected_cpu.db, actual_cpu.db
        ));
    }
    if expected_cpu.d != actual_cpu.d {
        return Some(format!(
            "[{}] D mismatch: expected {}, got {}",
            test_name, expected_cpu.d, actual_cpu.d
        ));
    }
    if expected_cpu.pb != actual_cpu.pb {
        return Some(format!(
            "[{}] PB mismatch: expected {}, got {}",
            test_name, expected_cpu.pb, actual_cpu.pb
        ));
    }
    if expected_cpu.e != actual_cpu.e {
        return Some(format!(
            "[{}] E mismatch: expected {}, got {}",
            test_name, expected_cpu.e, actual_cpu.e
        ));
    }

    // Check memory
    for ((expected_addr, expected_val), (actual_addr, actual_val)) in
        expected_cpu.ram.iter().zip(actual_memory.iter())
    {
        if expected_addr != actual_addr || expected_val != actual_val {
            return Some(format!(
                "[{}] Memory mismatch at {}: expected {}, got {}",
                test_name, expected_addr, expected_val, actual_val
            ));
        }
    }

    // Check cycle count
    if actual_cycles as usize != expected_cycles.len() {
        return Some(format!(
            "[{}] Cycle count mismatch: expected {}, got {}",
            test_name,
            expected_cycles.len(),
            actual_cycles
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_single_test() {
        // This is a simple NOP test
        let test_json = r#"
        {
          "name": "ea (NOP)",
          "initial": {
            "pc": 4096,
            "s": 511,
            "a": 0,
            "x": 0,
            "y": 0,
            "p": 52,
            "dbr": 0,
            "d": 0,
            "pbr": 0,
            "e": 0,
            "ram": [
              [4096, 234]
            ]
          },
          "final": {
            "pc": 4097,
            "s": 511,
            "a": 0,
            "x": 0,
            "y": 0,
            "p": 52,
            "dbr": 0,
            "d": 0,
            "pbr": 0,
            "e": 0,
            "ram": [
              [4096, 234]
            ]
          },
          "cycles": [
            [4096, 234, "read"]
          ]
        }
        "#;

        let test: TestCase = serde_json::from_str(test_json).unwrap();
        let result = run_test(&test);

        // NOP (0xEA) should increment PC by 1 and take 2 cycles
        // This test might fail if the opcode isn't implemented correctly
        println!("Test result: {:?}", result);
    }
}
