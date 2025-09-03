//! Simple OWL2 conformance test to demonstrate the reasoner's capabilities.

use owl2_rs::api::{load_ontology, Reasoner};

fn main() {
    println!("OWL2 Conformance Test");
    println!("====================");
    
    // Create a simple ontology with subsumption: Student ⊑ Person
    let ontology_str = "Ontology(<http://example.com/test> SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>)) ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>)))";
    
    // Load the ontology
    let ontology = load_ontology(ontology_str).expect("Failed to load ontology");
    println!("Loaded ontology with {} axioms", ontology.axioms.len());
    
    // Create a reasoner
    let mut reasoner = Reasoner::new(ontology);
    
    // Check consistency
    let is_consistent = reasoner.is_consistent();
    println!("Ontology is consistent: {}", is_consistent);
    assert!(is_consistent, "Ontology should be consistent");
    
    // Classify the ontology
    let hierarchy = reasoner.classify();
    println!("Classification completed");
    println!("Found {} superclasses", hierarchy.superclasses.len());
    println!("Found {} subclasses", hierarchy.subclasses.len());
    
    // Realize individuals
    let individual_types = reasoner.realize();
    println!("Realization completed");
    println!("Found types for {} individuals", individual_types.len());
    
    println!("\nTest completed successfully!");
}
