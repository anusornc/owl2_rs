//! Test runner example for OWL2 conformance test cases.
//!
//! This example demonstrates how to use the test runner to load and run OWL2 conformance test cases.

use owl2_rs::test_runner::run_owl2_test_case;
use std::path::Path;

fn main() {
    println!("OWL2 Test Runner Example");
    println!("======================");
    
    // Run a simple test case
    let test_file_path = Path::new("test_cases/sample_test.ofn");
    if test_file_path.exists() {
        match run_owl2_test_case(test_file_path) {
            Ok(()) => {
                println!("Test case completed successfully!");
            }
            Err(e) => {
                eprintln!("Failed to run test case: {}", e);
            }
        }
    } else {
        println!("Test file {:?} does not exist", test_file_path);
    }
}