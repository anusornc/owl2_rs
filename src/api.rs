//! # Public API for owl2_rs
//!
//! This module provides a clean, easy-to-use public API for the owl2_rs library.
//!
//! The API module is the main entry point for most users of the owl2_rs library.
//! It provides functions for loading ontologies and a Reasoner struct for
//! performing reasoning tasks.
//!
//! ## Error Handling
//!
//! All functions in this module return a Result type with the Owl2RsError enum
//! for error handling. This enum provides detailed information about what went
//! wrong during parsing or I/O operations.

use crate::{
    parser::OWLParser,
    reasoner::TableauReasoner,
    Ontology,
};
use std::{path::Path, io};
use thiserror::Error;

/// Errors that can occur when working with owl2_rs.
///
/// This enum provides detailed error information for different types of failures
/// that can occur when working with OWL 2 ontologies.
#[derive(Error, Debug)]
pub enum Owl2RsError {
    /// An error occurred during parsing.
    ///
    /// This error is returned when the OWL 2 parser encounters invalid syntax
    /// or other parsing issues.
    #[error("Parsing error: {0}")]
    ParsingError(#[from] Box<pest::error::Error<crate::parser::Rule>>),
    
    /// An I/O error occurred.
    ///
    /// This error is returned when there are issues reading from or writing to
    /// files or other I/O operations.
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
}

/// Loads an ontology from a string in OWL 2 Functional-Style Syntax.
///
/// This function parses an OWL 2 ontology represented as a string in
/// Functional-Style Syntax and returns an Ontology struct.
///
/// # Arguments
///
/// * `input` - A string containing the ontology in OWL 2 Functional-Style Syntax.
///
/// # Returns
///
/// * `Ok(Ontology)` - The parsed ontology.
/// * `Err(Owl2RsError)` - An error if parsing fails.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::api::load_ontology;
///
/// let ontology_str = r#"Ontology(<http://example.com/ontology>
///   SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
/// )"#;
///
/// let ontology = load_ontology(ontology_str)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_ontology(input: &str) -> Result<Ontology, Owl2RsError> {
    let parsed_ontology = OWLParser::parse_ontology(input);
    match parsed_ontology {
        Ok(ontology) => Ok(ontology),
        Err(e) => Err(Owl2RsError::ParsingError(e)),
    }
}

/// Loads an ontology from a file containing OWL 2 Functional-Style Syntax.
///
/// # Arguments
///
/// * `path` - The path to the file containing the ontology.
///
/// # Returns
///
/// * `Ok(Ontology)` - The parsed ontology.
/// * `Err(Owl2RsError)` - An error if reading the file or parsing fails.
///
/// # Examples
///
/// ```rust,ignore
/// use owl2_rs::api::load_ontology_from_file;
/// use std::path::Path;
///
/// let ontology = load_ontology_from_file(Path::new("ontology.ofn"))?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn load_ontology_from_file(path: &Path) -> Result<Ontology, Owl2RsError> {
    let content = std::fs::read_to_string(path)?;
    load_ontology(&content)
}

/// A reasoner for OWL 2 ontologies.
///
/// Provides functionality for checking consistency, classifying ontologies,
/// realizing individuals, and checking instance relationships.
pub struct Reasoner {
    /// The underlying tableau reasoner.
    tableau_reasoner: TableauReasoner,
}

impl Reasoner {
    /// Creates a new reasoner for the given ontology.
    ///
    /// # Arguments
    ///
    /// * `ontology` - The ontology to reason about.
    ///
    /// # Returns
    ///
    /// A new reasoner instance.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use owl2_rs::api::{load_ontology, Reasoner};
    ///
    /// let ontology_str = r#"Ontology(<http://example.com/ontology>
    ///   SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
    /// )"#;
    ///
    /// let ontology = load_ontology(ontology_str).unwrap();
    /// let reasoner = Reasoner::new(ontology);
    /// ```
    pub fn new(ontology: Ontology) -> Self {
        Reasoner {
            tableau_reasoner: TableauReasoner::new(ontology),
        }
    }

    /// Checks if the ontology is consistent (satisfiable).
    ///
    /// An ontology is consistent if it has at least one model, i.e., there exists
    /// an interpretation that satisfies all the axioms in the ontology.
    ///
    /// # Returns
    ///
    /// * `true` - If the ontology is consistent.
    /// * `false` - If the ontology is inconsistent.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use owl2_rs::api::{load_ontology, Reasoner};
    ///
    /// let ontology_str = r#"Ontology(<http://example.com/ontology>
    ///   ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
    /// )"#;
    ///
    /// let ontology = load_ontology(ontology_str).unwrap();
    /// let mut reasoner = Reasoner::new(ontology);
    /// let is_consistent = reasoner.is_consistent();
    /// assert!(is_consistent);
    /// ```
    pub fn is_consistent(&mut self) -> bool {
        self.tableau_reasoner.is_consistent()
    }

    /// Computes the class hierarchy for the ontology.
    ///
    /// This method computes the subsumption relationships between classes in the ontology.
    ///
    /// # Returns
    ///
    /// The computed class hierarchy.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use owl2_rs::api::{load_ontology, Reasoner};
    ///
    /// let ontology_str = r#"Ontology(<http://example.com/ontology>
    ///   SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
    /// )"#;
    ///
    /// let ontology = load_ontology(ontology_str).unwrap();
    /// let mut reasoner = Reasoner::new(ontology);
    /// let hierarchy = reasoner.classify();
    /// ```
    pub fn classify(&mut self) -> crate::reasoner::ClassHierarchy {
        self.tableau_reasoner.classify()
    }

    /// Finds the most specific types for all individuals in the ontology.
    ///
    /// This method determines the most specific classes that each individual belongs to.
    ///
    /// # Returns
    ///
    /// A mapping from individuals to their most specific types.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use owl2_rs::api::{load_ontology, Reasoner};
    /// use std::collections::HashMap;
    ///
    /// let ontology_str = r#"Ontology(<http://example.com/ontology>
    ///   ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
    /// )"#;
    ///
    /// let ontology = load_ontology(ontology_str).unwrap();
    /// let mut reasoner = Reasoner::new(ontology);
    /// let individual_types = reasoner.realize();
    /// ```
    pub fn realize(&mut self) -> std::collections::HashMap<crate::Individual, crate::reasoner::IndividualTypes> {
        self.tableau_reasoner.realize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_ontology() {
        let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;
        
        let ontology = load_ontology(ontology_str).unwrap();
        assert_eq!(ontology.axioms.len(), 1);
    }

    #[test]
    fn test_reasoner_creation() {
        let ontology_str = r#"Ontology(<http://example.com/ontology>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;
        
        let ontology = load_ontology(ontology_str).unwrap();
        let mut reasoner = Reasoner::new(ontology);
        
        assert!(reasoner.is_consistent());
    }
}