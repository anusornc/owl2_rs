#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::{load_ontology, Reasoner};

    #[test]
    fn test_api_load_ontology() {
        let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;
        
        let ontology = load_ontology(ontology_str).unwrap();
        assert_eq!(ontology.axioms.len(), 1);
    }

    #[test]
    fn test_api_reasoner_creation() {
        let ontology_str = r#"Ontology(<http://example.com/ontology>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;
        
        let ontology = load_ontology(ontology_str).unwrap();
        let mut reasoner = Reasoner::new(ontology);
        
        assert!(reasoner.is_consistent());
    }
}