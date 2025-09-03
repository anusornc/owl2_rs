//! # Integration Test for GS1 and EPCIS Ontologies
//!
//! This example demonstrates how to use the owl2_rs library with GS1 and EPCIS ontologies
//! for supply chain management applications.

use owl2_rs::api::{load_ontology_from_file, Reasoner};
use std::path::Path;
use std::time::Instant;

fn main() {
    println!("owl2_rs Integration Test - GS1 and EPCIS Ontologies");
    println!("===================================================");
    
    // Test GS1 ontology
    println!("\n1. Testing GS1 ontology:");
    test_gs1_ontology();
    
    // Test EPCIS ontology
    println!("\n2. Testing EPCIS ontology:");
    test_epcis_ontology();
    
    println!("\nIntegration test completed successfully!");
}

fn test_gs1_ontology() {
    let path = Path::new("test_cases/gs1_test.ofn");
    
    // Parse the ontology
    let start = Instant::now();
    let ontology = load_ontology_from_file(&path).expect("Failed to load GS1 ontology");
    let parse_duration = start.elapsed();
    
    println!("  Parsed GS1 ontology with {} axioms in {:?}", ontology.axioms.len(), parse_duration);
    
    // Check consistency
    let start = Instant::now();
    let mut reasoner = Reasoner::new(ontology);
    let is_consistent = reasoner.is_consistent();
    let consistency_duration = start.elapsed();
    
    println!("  GS1 ontology is consistent: {} (checked in {:?})", is_consistent, consistency_duration);
    
    // Classify the ontology
    let start = Instant::now();
    let hierarchy = reasoner.classify();
    let classification_duration = start.elapsed();
    
    println!("  Computed class hierarchy in {:?}", classification_duration);
    println!("  Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("  Found {} classes with subclasses", hierarchy.subclasses.len());
    
    // Realize individuals
    let start = Instant::now();
    let individual_types = reasoner.realize();
    let realization_duration = start.elapsed();
    
    println!("  Realized individuals in {:?}", realization_duration);
    println!("  Found types for {} individuals", individual_types.len());
}

fn test_epcis_ontology() {
    let path = Path::new("test_cases/epcis_test.ofn");
    
    // Parse the ontology
    let start = Instant::now();
    let ontology = load_ontology_from_file(&path).expect("Failed to load EPCIS ontology");
    let parse_duration = start.elapsed();
    
    println!("  Parsed EPCIS ontology with {} axioms in {:?}", ontology.axioms.len(), parse_duration);
    
    // Check consistency
    let start = Instant::now();
    let mut reasoner = Reasoner::new(ontology);
    let is_consistent = reasoner.is_consistent();
    let consistency_duration = start.elapsed();
    
    println!("  EPCIS ontology is consistent: {} (checked in {:?})", is_consistent, consistency_duration);
    
    // Classify the ontology
    let start = Instant::now();
    let hierarchy = reasoner.classify();
    let classification_duration = start.elapsed();
    
    println!("  Computed class hierarchy in {:?}", classification_duration);
    println!("  Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("  Found {} classes with subclasses", hierarchy.subclasses.len());
    
    // Realize individuals
    let start = Instant::now();
    let individual_types = reasoner.realize();
    let realization_duration = start.elapsed();
    
    println!("  Realized individuals in {:?}", realization_duration);
    println!("  Found types for {} individuals", individual_types.len());
}