//! Integration tests for the owl2_rs library.
//!
//! These tests use standard OWL2 conformance test cases to verify the correctness
//! of the parser and reasoner.

use owl2_rs::{
    parser::OWLParser,
    reasoner::TableauReasoner,
};

/// A test case for OWL2 reasoning.
#[derive(Debug)]
struct OWL2TestCase {
    /// The name of the test case
    name: String,
    /// The ontology as a string
    ontology_str: String,
    /// Expected consistency result
    expected_consistent: bool,
}

/// Runs an OWL2 test case.
fn run_owl2_test_case(test_case: OWL2TestCase) {
    println!("Running test case: {}", test_case.name);
    
    // Parse the ontology
    let ontology = OWLParser::parse_ontology(&test_case.ontology_str)
        .expect(&format!("Failed to parse ontology for test case: {}", test_case.name));
    
    // Create a reasoner
    let mut reasoner = TableauReasoner::new(ontology);
    
    // Check consistency
    let is_consistent = reasoner.is_consistent();
    assert_eq!(is_consistent, test_case.expected_consistent,
        "Consistency check failed for test case: {}. Expected: {}, Got: {}",
        test_case.name, test_case.expected_consistent, is_consistent);
    
    println!("Test case {} passed!", test_case.name);
}

#[test]
fn test_basic_consistency() {
    let test_case = OWL2TestCase {
        name: "Basic Consistency Test".to_string(),
        ontology_str: r#"Ontology(<http://example.com/test>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#.to_string(),
        expected_consistent: true,
    };
    
    run_owl2_test_case(test_case);
}

#[test]
fn test_inconsistent_ontology() {
    let test_case = OWL2TestCase {
        name: "Inconsistent Ontology Test".to_string(),
        ontology_str: r#"Ontology(<http://example.com/test>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ClassAssertion(ObjectComplementOf(Class(<http://example.com/Student>)) NamedIndividual(<http://example.com/john>))
)"#.to_string(),
        expected_consistent: false,
    };
    
    run_owl2_test_case(test_case);
}

#[test]
fn test_subclass_relationship() {
    let test_case = OWL2TestCase {
        name: "Subclass Relationship Test".to_string(),
        ontology_str: r#"Ontology(<http://example.com/test>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#.to_string(),
        expected_consistent: true,
    };
    
    run_owl2_test_case(test_case);
}