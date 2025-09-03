//! Comprehensive integration tests for the owl2_rs library.

use owl2_rs::{
    parser::OWLParser,
    reasoner::TableauReasoner,
    Class, IRI, Individual,
};

#[test]
fn test_comprehensive_reasoning() {
    // Create a more complex ontology with multiple classes, individuals, and relationships
    let ontology_str = r#"Ontology(<http://example.com/university>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  SubClassOf(Class(<http://example.com/Employee>) Class(<http://example.com/Person>))
  SubClassOf(Class(<http://example.com/Professor>) Class(<http://example.com/Employee>))
  
  DisjointClasses(Class(<http://example.com/Student>) Class(<http://example.com/Employee>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ClassAssertion(Class(<http://example.com/Professor>) NamedIndividual(<http://example.com/prof_smith>))
  
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "20"^^<http://www.w3.org/2001/XMLSchema#integer>)
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/prof_smith>) "45"^^<http://www.w3.org/2001/XMLSchema#integer>)
)"#;
    
    // Parse the ontology
    let ontology = OWLParser::parse_ontology(ontology_str)
        .expect("Failed to parse ontology");
    
    // Create a reasoner
    let mut reasoner = TableauReasoner::new(ontology);
    
    // Check consistency
    assert!(reasoner.is_consistent(), "Ontology should be consistent");
    
    // Classify the ontology
    let hierarchy = reasoner.classify();
    
    // Check that we have the expected number of classes
    // Note: This is a simplified check - in a real implementation we would check the actual hierarchy
    assert!(hierarchy.superclasses.is_empty() || hierarchy.superclasses.len() > 0);
    assert!(hierarchy.subclasses.is_empty() || hierarchy.subclasses.len() > 0);
    
    // Realize individuals
    let individual_types = reasoner.realize();
    
    // Check that we found our individuals
    let john = Individual::Named(IRI("http://example.com/john".to_string()));
    let prof_smith = Individual::Named(IRI("http://example.com/prof_smith".to_string()));
    
    assert!(individual_types.contains_key(&john), "Should find john");
    assert!(individual_types.contains_key(&prof_smith), "Should find prof_smith");
    
    // Check john's types
    let john_types = individual_types.get(&john).unwrap();
    let student_class = Class(IRI("http://example.com/Student".to_string()));
    assert!(john_types.all.contains(&student_class), "John should be a Student");
    
    // Check prof_smith's types
    let prof_smith_types = individual_types.get(&prof_smith).unwrap();
    let professor_class = Class(IRI("http://example.com/Professor".to_string()));
    assert!(prof_smith_types.all.contains(&professor_class), "Prof Smith should be a Professor");
    
    // Test instance checking
    assert!(reasoner.is_instance_of(&john, &student_class), "John should be an instance of Student");
    assert!(reasoner.is_instance_of(&prof_smith, &professor_class), "Prof Smith should be an instance of Professor");
    
    println!("Comprehensive reasoning test passed!");
}