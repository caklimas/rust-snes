// Integration tests for the common test infrastructure
mod common;

use std::fs;

use crate::common::test_runner::{load_tests_from_file, run_test};

#[test]
fn test_json_file() {
    let root =
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("external/SingleStepTests/v1");
    let entries = fs::read_dir(root).unwrap();
    let mut files: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();
    files.sort_by_key(|entry| entry.file_name());

    // 44.e/n and 54.e/n (MVN/MVP) are skipped because the test fixtures are
    // known-broken: the 100-cycle capture limit cuts off mid-instruction
    // (each iteration is 7 cycles, so 100 isn't a clean multiple), leaving A
    // never reaching 0xFFFF and PC off by 1. See:
    // https://github.com/SingleStepTests/65816/issues/8
    let files_to_skip = ["44.e.json", "44.n.json", "54.e.json", "54.n.json"];

    // Skip the first N files to start testing from a specific opcode.
    // Adjust this number to resume where you left off.
    let skip_count = 0;

    for file in files
        .iter()
        .filter(|f| !files_to_skip.contains(&f.file_name().to_str().unwrap_or("")))
        .skip(skip_count)
    {
        let path = file.path();
        println!("Testing file {}", path.display());
        let test_cases = load_tests_from_file(path).unwrap();

        for test_case in test_cases.iter() {
            let test_result = run_test(test_case);
            if !test_result.passed {
                println!("{}", test_result.failure_reason.unwrap())
            }

            assert!(test_result.passed)
        }
    }

    println!("All test cases passed!");
}
