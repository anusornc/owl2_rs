//! Comprehensive OWL 2 Profile Tests
//!
//! These tests verify the OWL 2 profile checking functionality.

use owl2_rs::{
    api::load_ontology,
    owl2_profile::{check_profile_compliance, OwlProfile}
};

#[test]
fn test_el_profile_compliant_ontology() {
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
  DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
  
  FunctionalDataProperty(DataProperty(<http://example.com/hasAge>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ObjectPropertyAssertion(ObjectProperty(<http://example.com/hasParent>) NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/mary>))
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::EL);
    
    assert!(result.conforms, "EL-compliant ontology should conform to EL profile. Violations: {:?}", result.violations);
    assert!(result.violations.is_empty());
}

#[test]
fn test_el_profile_violation_union() {
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::EL);
    
    assert!(!result.conforms, "Ontology with union should not conform to EL profile");
    assert!(!result.violations.is_empty());
}

#[test]
fn test_el_profile_violation_complement() {
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) ObjectComplementOf(Class(<http://example.com/Employee>)))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::EL);
    
    assert!(!result.conforms, "Ontology with complement should not conform to EL profile");
    assert!(!result.violations.is_empty());
}

#[test]
fn test_el_profile_violation_universal() {
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Manager>) ObjectAllValuesFrom(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Company>)))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::EL);
    
    assert!(!result.conforms, "Ontology with universal restriction should not conform to EL profile");
    assert!(!result.violations.is_empty());
}

#[test]
fn test_rl_profile_with_has_self() {
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Narcissist>) ObjectHasSelf(ObjectProperty(<http://example.com/loves>)))
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let el_result = check_profile_compliance(&ontology, OwlProfile::EL);
    // ObjectHasSelf is not allowed in EL
    assert!(!el_result.conforms);
    
    // For RL, we would need to implement the RL checking logic
    // For now, we'll just test that the ontology parses correctly
    assert_eq!(ontology.axioms.len(), 1);
}

#[test]
fn test_complex_el_compliant_ontology() {
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(
    Class(<http://example.com/Manager>)
    ObjectIntersectionOf(
      Class(<http://example.com/Employee>)
      ObjectSomeValuesFrom(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Company>))
    )
  )
  
  SubClassOf(
    Class(<http://example.com/Student>)
    ObjectIntersectionOf(
      Class(<http://example.com/Person>)
      ObjectSomeValuesFrom(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
    )
  )
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
  DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
  
  FunctionalDataProperty(DataProperty(<http://example.com/hasAge>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ObjectPropertyAssertion(ObjectProperty(<http://example.com/hasParent>) NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/mary>))
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
)"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::EL);
    
    assert!(result.conforms, "Complex EL-compliant ontology should conform to EL profile. Violations: {:?}", result.violations);
    assert!(result.violations.is_empty());
}