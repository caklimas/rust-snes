// Integration tests for the common test infrastructure
mod common;

use std::fs;

use crate::common::test_runner::{load_tests_from_file, run_test};

#[test]
fn test_json_file() {
    let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("external/ProcessorTests/65816/v1");
    let entries = fs::read_dir(root).unwrap();
    let mut files: Vec<_> = entries.filter_map(|entry| entry.ok()).collect();

    // 3. Sort the vector of entries by their path.
    // The `sort_by_key` method is an efficient way to sort based on a specific field.
    files.sort_by_key(|entry| entry.file_name());

    for file in files.iter().skip(30) {
        let path = file.path();
        println!("Testing file {}", path.display());
        let test_cases = load_tests_from_file(path).unwrap();

        for test_case in test_cases.iter().filter(|x| x.name == "11 n 2" || true) {
            let test_result = run_test(test_case);
            if !test_result.passed {
                println!("{}", test_result.failure_reason.unwrap())
            }

            assert!(test_result.passed)
        }
    }

    println!("All test cases passed!");
}
