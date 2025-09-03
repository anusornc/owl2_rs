//! # owl2_rs Example
//!
//! This example demonstrates the basic functionality of the owl2_rs library.

use owl2_rs::{parser::OWLParser, reasoner::TableauReasoner};
use std::time::Instant;

fn main() {
    println!("OWL2_rs Example - Basic Reasoning");
    println!("==================================");
    
    // Example 1: Parse a simple ontology
    println!("\n1. Parsing a simple ontology:");
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  SubClassOf(Class(<http://example.com/Employee>) Class(<http://example.com/Person>))
  
  DisjointClasses(Class(<http://example.com/Student>) Class(<http://example.com/Employee>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Employee>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Organization>))
  
  DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
  DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
)"#;
    
    let start = Instant::now();
    let ontology = OWLParser::parse_ontology(ontology_str).expect("Failed to parse ontology");
    let parse_duration = start.elapsed();
    
    println!("  Parsed ontology with {} axioms in {:?}", ontology.axioms.len(), parse_duration);
    
    // Example 2: Check consistency
    println!("\n2. Checking consistency:");
    let start = Instant::now();
    let mut reasoner = TableauReasoner::new(ontology);
    let is_consistent = reasoner.is_consistent();
    let consistency_duration = start.elapsed();
    
    println!("  Ontology is consistent: {} (checked in {:?})", is_consistent, consistency_duration);
    
    // Example 3: Classify the ontology
    println!("\n3. Computing class hierarchy:");
    let start = Instant::now();
    let hierarchy = reasoner.classify();
    let classification_duration = start.elapsed();
    
    println!("  Computed class hierarchy in {:?}", classification_duration);
    println!("  Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("  Found {} classes with subclasses", hierarchy.subclasses.len());
    
    // Example 4: Realize individuals
    println!("\n4. Realizing individuals:");
    let start = Instant::now();
    let individual_types = reasoner.realize();
    let realization_duration = start.elapsed();
    
    println!("  Realized individuals in {:?}", realization_duration);
    println!("  Found types for {} individuals", individual_types.len());
    
    // Print information about the individuals
    for (individual, types) in individual_types.iter() {
        println!("  Individual: {:?}", individual);
        println!("    Most specific types: {:?}", types.most_specific);
        println!("    All types: {:?}", types.all);
    }
    
    // Example 5: Instance checking
    println!("\n5. Instance checking:");
    let start = Instant::now();
    let individual_john = owl2_rs::Individual::Named(owl2_rs::IRI("http://example.com/john".to_string()));
    let class_student = owl2_rs::Class(owl2_rs::IRI("http://example.com/Student".to_string()));
    let class_person = owl2_rs::Class(owl2_rs::IRI("http://example.com/Person".to_string()));
    
    let is_john_student = reasoner.is_instance_of(&individual_john, &class_student);
    let is_john_person = reasoner.is_instance_of(&individual_john, &class_person);
    let instance_checking_duration = start.elapsed();
    
    println!("  Instance checking completed in {:?}", instance_checking_duration);
    println!("  Is john an instance of Student? {}", is_john_student);
    println!("  Is john an instance of Person? {}", is_john_person);
    
    println!("\nExample completed successfully!");
}