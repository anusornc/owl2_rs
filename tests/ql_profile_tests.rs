//! # OWL 2 QL Profile Tests
//!
//! These tests verify the OWL 2 QL profile checking functionality.

use owl2_rs::{
    api::load_ontology,
    owl2_profile::{check_profile_compliance, OwlProfile},
    Class, ClassExpression, DataProperty, Datatype, IRI, 
    ObjectProperty, ObjectPropertyExpression, Individual, DataRange
};

#[test]
fn test_ql_profile_compliant_ontology() {
    // Test a simple QL-compliant ontology
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
  DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ObjectPropertyAssertion(ObjectProperty(<http://example.com/hasParent>) NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/mary>))
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(result.conforms, "QL-compliant ontology should conform to QL profile. Violations: {:?}", result.violations);
    assert!(result.violations.is_empty());
}

#[test]
fn test_ql_profile_violation_disjoint_union() {
    // Test that DisjointUnion is not allowed in QL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  DisjointUnion(Class(<http://example.com/Person>) Class(<http://example.com/Student>) Class(<http://example.com/Employee>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(!result.conforms, "Ontology with DisjointUnion should not conform to QL profile");
    assert!(result.violations.iter().any(|v| v.contains("DisjointUnion")));
}

#[test]
fn test_ql_profile_violation_transitive_property() {
    // Test that TransitiveObjectProperty is not allowed in QL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  TransitiveObjectProperty(ObjectProperty(<http://example.com/ancestorOf>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(!result.conforms, "Ontology with TransitiveObjectProperty should not conform to QL profile");
    assert!(result.violations.iter().any(|v| v.contains("TransitiveObjectProperty")));
}

#[test]
fn test_ql_profile_violation_functional_property() {
    // Test that FunctionalObjectProperty is not allowed in QL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  FunctionalObjectProperty(ObjectProperty(<http://example.com/hasBirthMother>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(!result.conforms, "Ontology with FunctionalObjectProperty should not conform to QL profile");
    assert!(result.violations.iter().any(|v| v.contains("FunctionalObjectProperty")));
}

#[test]
fn test_ql_profile_violation_same_individual() {
    // Test that SameIndividual is not allowed in QL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SameIndividual(NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/johnny>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(!result.conforms, "Ontology with SameIndividual should not conform to QL profile");
    assert!(result.violations.iter().any(|v| v.contains("SameIndividual")));
}

#[test]
fn test_ql_profile_subclass_restrictions() {
    // Test QL subclass expression restrictions
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(!result.conforms, "Ontology with union in subclass position should not conform to QL profile");
    assert!(result.violations.iter().any(|v| v.contains("subclass expression")));
}

#[test]
fn test_ql_profile_superclass_restrictions() {
    // Test QL superclass expression restrictions
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) ObjectUnionOf(Class(<http://example.com/Person>) Class(<http://example.com/Worker>)))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::QL);
    
    assert!(!result.conforms, "Ontology with union in superclass position should not conform to QL profile");
    assert!(result.violations.iter().any(|v| v.contains("superclass expression")));
}