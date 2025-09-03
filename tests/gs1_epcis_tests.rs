//! # GS1 and EPCIS Tests
//!
//! This module contains tests for parsing and reasoning with GS1 and EPCIS ontologies.

use owl2_rs::api::{load_ontology_from_file, Reasoner};
use std::path::Path;

#[test]
fn test_gs1_ontology_parsing() {
    let path = Path::new("test_cases/gs1_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load GS1 ontology");
    
    // Check that we have the expected number of axioms
    assert!(ontology.axioms.len() > 10);
    
    println!("Successfully parsed GS1 ontology with {} axioms", ontology.axioms.len());
}

#[test]
fn test_gs1_ontology_consistency() {
    let path = Path::new("test_cases/gs1_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load GS1 ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Check that the ontology is consistent
    assert!(reasoner.is_consistent());
    
    println!("GS1 ontology is consistent");
}

#[test]
fn test_gs1_ontology_classification() {
    let path = Path::new("test_cases/gs1_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load GS1 ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Compute the class hierarchy
    let hierarchy = reasoner.classify();
    
    // We can at least check that the function runs without error
    println!("Computed class hierarchy for GS1 ontology:");
    println!("  Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("  Found {} classes with subclasses", hierarchy.subclasses.len());
}

#[test]
fn test_epcis_ontology_parsing() {
    let path = Path::new("test_cases/epcis_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load EPCIS ontology");
    
    // Check that we have the expected number of axioms
    assert!(ontology.axioms.len() > 10);
    
    println!("Successfully parsed EPCIS ontology with {} axioms", ontology.axioms.len());
}

#[test]
fn test_epcis_ontology_consistency() {
    let path = Path::new("test_cases/epcis_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load EPCIS ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Check that the ontology is consistent
    assert!(reasoner.is_consistent());
    
    println!("EPCIS ontology is consistent");
}

#[test]
fn test_epcis_ontology_classification() {
    let path = Path::new("test_cases/epcis_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load EPCIS ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Compute the class hierarchy
    let hierarchy = reasoner.classify();
    
    // We can at least check that the function runs without error
    println!("Computed class hierarchy for EPCIS ontology:");
    println!("  Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("  Found {} classes with subclasses", hierarchy.subclasses.len());
}

#[test]
fn test_gs1_reasoning() {
    let path = Path::new("test_cases/gs1_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load GS1 ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Realize individuals
    let individual_types = reasoner.realize();
    
    // Check that we have realized types for our individuals
    assert!(!individual_types.is_empty());
    
    println!("Realized individuals in GS1 ontology:");
    println!("  Found types for {} individuals", individual_types.len());
    
    // Check specific inferences
    // For example, since product1 has a manufacturer (company1), and company1 has a location,
    // we should be able to infer some relationships
}

#[test]
fn test_epcis_reasoning() {
    let path = Path::new("test_cases/epcis_test.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load EPCIS ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Realize individuals
    let individual_types = reasoner.realize();
    
    // Check that we have realized types for our individuals
    assert!(!individual_types.is_empty());
    
    println!("Realized individuals in EPCIS ontology:");
    println!("  Found types for {} individuals", individual_types.len());
}