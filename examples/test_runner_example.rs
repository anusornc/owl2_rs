//! Example demonstrating the OWL2 test runner.
//!
//! This example shows how to use the test runner to load and run OWL2 conformance test cases.

use owl2_rs::test_runner::run_owl2_test_suite;
use std::path::Path;

fn main() {
    println!("OWL2 Test Runner Example");
    println!("======================");
    
    // Run the OWL2 conformance test suite
    let test_dir_path = Path::new("test_suites/w3c_owl2_tests");
    if test_dir_path.exists() {
        match run_owl2_test_suite(test_dir_path) {
            Ok(passed_count) => {
                println!("Test suite completed successfully!");
                println!("Passed {} tests", passed_count);
            }
            Err(e) => {
                eprintln!("Failed to run test suite: {}", e);
            }
        }
    } else {
        println!("Test directory {:?} does not exist", test_dir_path);
        println!("Please download the OWL2 conformance test suite first.");
    }
}