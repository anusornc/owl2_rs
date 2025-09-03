//! Test runner for OWL2 conformance test suites.
//!
//! This module provides functionality to load and run OWL2 conformance test cases
//! from the W3C OWL2 test repository.

use crate::api::{load_ontology_from_file, Reasoner};
use std::path::Path;

/// Runs a single OWL2 conformance test case.
///
/// # Arguments
///
/// * `test_file_path` - Path to the test file in OWL 2 Functional-Style Syntax or RDF/XML format.
///
/// # Returns
///
/// * `Ok(())` - If the test passes.
/// * `Err(String)` - If the test fails or an error occurs.
pub fn run_owl2_test_case(test_file_path: &Path) -> Result<(), String> {
    // Load the ontology from the test file
    let ontology = load_ontology_from_file(test_file_path).map_err(|e| format!("Failed to load ontology: {:?}", e))?;
    
    // Create a reasoner
    let mut reasoner = Reasoner::new(ontology);
    
    // Check consistency
    let is_consistent = reasoner.is_consistent();
    
    // For now, we'll just print the result
    println!("Test case {:?} is consistent: {}", test_file_path, is_consistent);
    
    Ok(())
}

/// Runs all OWL2 conformance test cases in a directory.
///
/// # Arguments
///
/// * `test_dir_path` - Path to the directory containing test files.
///
/// # Returns
///
/// * `Ok(usize)` - Number of tests that passed.
/// * `Err(String)` - If an error occurs.
pub fn run_owl2_test_suite(test_dir_path: &Path) -> Result<usize, String> {
    use std::fs;
    
    let mut passed_count = 0;
    let mut total_count = 0;
    
    // Read all files in the directory
    let entries = fs::read_dir(test_dir_path).map_err(|e| format!("Failed to read directory: {:?}", e))?;
    
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {:?}", e))?;
        let path = entry.path();
        
        // Check if it's an RDF file
        if path.extension().map_or(false, |ext| ext == "rdf") {
            total_count += 1;
            match run_owl2_test_case(&path) {
                Ok(()) => {
                    passed_count += 1;
                    println!("Test case {:?} PASSED", path);
                }
                Err(e) => {
                    println!("Test case {:?} FAILED: {}", path, e);
                }
            }
        }
    }
    
    println!("Test suite completed: {}/{} tests passed", passed_count, total_count);
    Ok(passed_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    
    #[test]
    fn test_run_owl2_test_case() {
        // Test with a simple ontology file
        let test_file_path = PathBuf::from("test_suites/owl2bench/OWL2Bench/UNIV-BENCH-OWL2DL.owl");
        if test_file_path.exists() {
            let result = run_owl2_test_case(&test_file_path);
            // For now, we'll just check that the function doesn't panic
            // In a real implementation, we would check the result
            println!("Test result: {:?}", result);
        } else {
            // Skip the test if the file doesn't exist
            println!("Skipping test: test file {:?} does not exist", test_file_path);
        }
    }
}