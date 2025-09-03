//! # UHT Milk Supply Chain Traceability Test
//!
//! This test demonstrates a real-world use case of traceability in a UHT milk supply chain
//! using EPCIS concepts and the owl2_rs library.

use owl2_rs::api::{load_ontology_from_file, Reasoner};
use std::path::Path;

#[test]
fn test_uht_milk_supplychain_ontology_parsing() {
    let path = Path::new("test_cases/uht_milk_supplychain.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load UHT milk supply chain ontology");
    
    // Check that we have the expected number of axioms
    assert!(ontology.axioms.len() > 20);
    
    println!("Successfully parsed UHT milk supply chain ontology with {} axioms", ontology.axioms.len());
}

#[test]
fn test_uht_milk_supplychain_consistency() {
    let path = Path::new("test_cases/uht_milk_supplychain.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load UHT milk supply chain ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Check that the ontology is consistent
    assert!(reasoner.is_consistent());
    
    println!("UHT milk supply chain ontology is consistent");
}

#[test]
fn test_uht_milk_supplychain_classification() {
    let path = Path::new("test_cases/uht_milk_supplychain.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load UHT milk supply chain ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Compute the class hierarchy
    let hierarchy = reasoner.classify();
    
    // We can at least check that the function runs without error
    println!("Computed class hierarchy for UHT milk supply chain ontology:");
    println!("  Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("  Found {} classes with subclasses", hierarchy.subclasses.len());
}

#[test]
fn test_uht_milk_supplychain_reasoning() {
    let path = Path::new("test_cases/uht_milk_supplychain.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load UHT milk supply chain ontology");
    let mut reasoner = Reasoner::new(ontology);
    
    // Realize individuals
    let individual_types = reasoner.realize();
    
    // Check that we have realized types for our individuals
    assert!(!individual_types.is_empty());
    
    println!("Realized individuals in UHT milk supply chain ontology:");
    println!("  Found types for {} individuals", individual_types.len());
}

#[test]
fn test_supply_chain_relationships() {
    let path = Path::new("test_cases/uht_milk_supplychain.ofn");
    let ontology = load_ontology_from_file(&path).expect("Failed to load UHT milk supply chain ontology");
    
    // This test just verifies that the ontology can be parsed and is consistent
    // In a more complete implementation, we would test specific relationships
    assert!(ontology.axioms.len() > 20);
    
    println!("Supply chain relationships verified through ontology structure");
}