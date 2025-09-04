//! # OWL 2 RL Profile Tests
//!
//! These tests verify the OWL 2 RL profile checking functionality.

use owl2_rs::{
    api::load_ontology,
    owl2_profile::{check_profile_compliance, OwlProfile},
};

#[test]
fn test_rl_profile_compliant_ontology() {
    // Test a simple RL-compliant ontology
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
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "RL-compliant ontology should conform to RL profile. Violations: {:?}", result.violations);
    assert!(result.violations.is_empty());
}

#[test]
fn test_rl_profile_violation_disjoint_union() {
    // Test that DisjointUnion is not allowed in RL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  DisjointUnion(Class(<http://example.com/Person>) Class(<http://example.com/Student>) Class(<http://example.com/Employee>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(!result.conforms, "Ontology with DisjointUnion should not conform to RL profile");
    assert!(result.violations.iter().any(|v| v.contains("DisjointUnion")));
}

#[test]
fn test_rl_profile_violation_reflexive_property() {
    // Test that ReflexiveObjectProperty is not allowed in RL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  ReflexiveObjectProperty(ObjectProperty(<http://example.com/knows>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(!result.conforms, "Ontology with ReflexiveObjectProperty should not conform to RL profile");
    assert!(result.violations.iter().any(|v| v.contains("ReflexiveObjectProperty")));
}

#[test]
fn test_rl_profile_cardinality_restrictions() {
    // Test RL cardinality restrictions (only max 0 or 1 allowed)
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) ObjectMaxCardinality(1 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>)))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "Ontology with ObjectMaxCardinality(1) should conform to RL profile. Violations: {:?}", result.violations);
    
    // Test with max 2 (should violate RL)
    let ontology_str2 = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) ObjectMaxCardinality(2 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>)))
)"#;
    
    let ontology2 = load_ontology(ontology_str2).expect("Failed to parse ontology");
    let result2 = check_profile_compliance(&ontology2, OwlProfile::RL);
    
    assert!(!result2.conforms, "Ontology with ObjectMaxCardinality(2) should not conform to RL profile");
}

#[test]
fn test_rl_profile_complex_subclass_expressions() {
    // Test RL subclass expression support
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(
    Class(<http://example.com/Student>)
    ObjectIntersectionOf(
      ObjectUnionOf(Class(<http://example.com/Person>) Class(<http://example.com/Worker>))
      ObjectOneOf(NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/jane>))
    )
  )
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "Complex RL subclass expressions should conform to RL profile. Violations: {:?}", result.violations);
}

#[test]
fn test_rl_profile_superclass_restrictions() {
    // Test RL superclass expression support
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(
    Class(<http://example.com/Student>)
    ObjectIntersectionOf(
      Class(<http://example.com/Person>)
      ObjectComplementOf(Class(<http://example.com/Employee>))
      ObjectAllValuesFrom(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Organization>))
      ObjectHasValue(ObjectProperty(<http://example.com/hasName>) NamedIndividual(<http://example.com/john>))
      ObjectMaxCardinality(1 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>))
    )
  )
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "RL superclass expressions should conform to RL profile. Violations: {:?}", result.violations);
}

#[test]
fn test_rl_profile_datatype_restrictions() {
    // Test RL datatype restrictions (no owl:real or owl:rational)
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  DataPropertyRange(DataProperty(<http://example.com/hasHeight>) Datatype(<http://www.w3.org/2001/XMLSchema#decimal>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "Standard datatypes should conform to RL profile. Violations: {:?}", result.violations);
}