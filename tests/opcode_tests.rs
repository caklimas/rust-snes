// Opcode tests - Run SingleStepTests JSON test files
mod common;

use common::test_runner::run_tests_from_file;

// Example: Run tests for a specific opcode
// To use this, you'd need to download the SingleStepTests JSON files first
// For example: https://github.com/SingleStepTests/65816/tree/main/v1

#[test]
#[ignore] // Ignored by default - run with: cargo test -- --ignored
fn test_nop_opcode() {
    // This assumes you have the test file at this location
    // Download from: https://github.com/SingleStepTests/65816/blob/main/v1/ea.json
    let test_file = "tests/data/ea.json";

    match run_tests_from_file(test_file) {
        Ok(summary) => {
            summary.print_summary();
            assert_eq!(summary.failed, 0, "Some tests failed");
        }
        Err(e) => {
            // File probably doesn't exist yet - that's ok for now
            println!("Skipping test: {}", e);
            println!("Download test files from: https://github.com/SingleStepTests/65816");
        }
    }
}

#[test]
#[ignore] // Run with: cargo test -- --ignored
fn test_lda_immediate() {
    // LDA immediate - opcode 0xA9
    let test_file = "tests/data/a9.json";

    match run_tests_from_file(test_file) {
        Ok(summary) => {
            summary.print_summary();
            assert_eq!(summary.failed, 0, "Some tests failed");
        }
        Err(e) => {
            println!("Skipping test: {}", e);
        }
    }
}

// You can add more tests for each opcode file here
// Each opcode has its own JSON file (e.g., a9.json, ad.json, etc.)
