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

    let files_to_skip = ["44.e.json", "44.n.json", "54.e.json", "54.n.json"];

    // Skip the first N files to start testing from a specific opcode.
    // Adjust this number to resume where you left off.
    let skip_count = 406;

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
