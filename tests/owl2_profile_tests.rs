//! # OWL 2 Profile Tests
//!
//! This module contains tests for checking OWL 2 profile compliance.
//!
//! OWL 2 defines several profiles that restrict the expressivity of the language
//! to achieve better computational properties:
//!
//! 1. **OWL 2 EL** - Designed for ontologies with large numbers of individuals
//! 2. **OWL 2 QL** - Designed for querying large databases
//! 3. **OWL 2 RL** - Designed for rule-based reasoning
//!
//! Each profile restricts the allowed constructs in an ontology.

#[cfg(test)]
mod tests {
    use owl2_rs::{
        api::load_ontology
    };

    /// Test that checks if an ontology conforms to OWL 2 EL profile
    #[test]
    fn test_owl2_el_profile() {
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
        assert_eq!(ontology.axioms.len(), 9);
        
        // This ontology should be parseable and consistent
        let mut reasoner = owl2_rs::api::Reasoner::new(ontology);
        assert!(reasoner.is_consistent());
    }

    /// Test that checks if an ontology conforms to OWL 2 RL profile
    #[test]
    fn test_owl2_rl_profile() {
        let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  SubClassOf(Class(<http://example.com/Narcissist>) ObjectHasSelf(ObjectProperty(<http://example.com/loves>)))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  ReflexiveObjectProperty(ObjectProperty(<http://example.com/loves>))
  SymmetricObjectProperty(ObjectProperty(<http://example.com/loves>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ObjectPropertyAssertion(ObjectProperty(<http://example.com/hasParent>) NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/mary>))
)"#;
        
        let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
        assert_eq!(ontology.axioms.len(), 8);
        
        // This ontology should be parseable and consistent
        let mut reasoner = owl2_rs::api::Reasoner::new(ontology);
        assert!(reasoner.is_consistent());
    }

    /// Test that shows a full OWL 2 ontology with constructs not allowed in profiles
    #[test]
    fn test_full_owl2_not_in_profiles() {
        let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
  
  SubClassOf(Class(<http://example.com/Student>) ObjectComplementOf(Class(<http://example.com/Employee>)))
  
  SubClassOf(Class(<http://example.com/Manager>) ObjectAllValuesFrom(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Company>)))
)"#;
        
        let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
        assert_eq!(ontology.axioms.len(), 3);
        
        // This ontology should be parseable and consistent
        let mut reasoner = owl2_rs::api::Reasoner::new(ontology);
        assert!(reasoner.is_consistent());
    }

    /// Test profile checking functionality
    #[test]
    fn test_profile_checking() {
        // Create a simple EL profile ontology
        let el_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;
        
        let el_ontology = load_ontology(el_ontology_str).expect("Failed to parse EL ontology");
        assert_eq!(el_ontology.axioms.len(), 4);
        
        // Create a full OWL 2 ontology with union (not in EL)
        let full_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
)"#;
        
        let full_ontology = load_ontology(full_ontology_str).expect("Failed to parse full ontology");
        assert_eq!(full_ontology.axioms.len(), 1);
    }

    /// Test QL profile checking
    #[test]
    fn test_ql_profile_checking() {
        use owl2_rs::owl2_profile::{check_profile_compliance, OwlProfile};

        // QL-compliant ontology
        let ql_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;
        
        let ql_ontology = load_ontology(ql_ontology_str).expect("Failed to parse QL ontology");
        let ql_result = check_profile_compliance(&ql_ontology, OwlProfile::QL);
        assert!(ql_result.conforms);

        // Non-QL compliant ontology (has TransitiveObjectProperty)
        let non_ql_ontology_str = r#"Ontology(<http://example.com/ontology>
  TransitiveObjectProperty(ObjectProperty(<http://example.com/ancestorOf>))
)"#;
        
        let non_ql_ontology = load_ontology(non_ql_ontology_str).expect("Failed to parse non-QL ontology");
        let non_ql_result = check_profile_compliance(&non_ql_ontology, OwlProfile::QL);
        assert!(!non_ql_result.conforms);
    }

    /// Test RL profile checking
    #[test]
    fn test_rl_profile_checking() {
        use owl2_rs::owl2_profile::{check_profile_compliance, OwlProfile};

        // RL-compliant ontology
        let rl_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;
        
        let rl_ontology = load_ontology(rl_ontology_str).expect("Failed to parse RL ontology");
        let rl_result = check_profile_compliance(&rl_ontology, OwlProfile::RL);
        assert!(rl_result.conforms);

        // Non-RL compliant ontology (has ReflexiveObjectProperty)
        let non_rl_ontology_str = r#"Ontology(<http://example.com/ontology>
  ReflexiveObjectProperty(ObjectProperty(<http://example.com/knows>))
)"#;
        
        let non_rl_ontology = load_ontology(non_rl_ontology_str).expect("Failed to parse non-RL ontology");
        let non_rl_result = check_profile_compliance(&non_rl_ontology, OwlProfile::RL);
        assert!(!non_rl_result.conforms);
    }
}