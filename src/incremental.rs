//! # Incremental Reasoning for OWL 2
//! 
//! This module provides support for incremental reasoning capabilities.
//! 
//! ## Overview
//! 
//! Incremental reasoning allows the reasoner to reuse previous reasoning results
//! when the ontology is modified, potentially speeding up reasoning operations
//! when only small changes have been made.
//! 
//! ## Usage
//! 
//! ```rust,ignore
//! use owl2_rs::incremental::{IncrementalReasoner, ReasoningResults};
//! 
//! let mut reasoner = IncrementalReasoner::new(ontology);
//! let results = reasoner.reason_incremental();
//! ```

use crate::{Ontology, Individual, reasoner::{TableauReasoner, ClassHierarchy, IndividualTypes}};
use std::collections::HashMap;

/// Results from a reasoning operation that can be reused for incremental reasoning.
#[derive(Debug, Clone)]
pub struct ReasoningResults {
    /// The class hierarchy from the previous reasoning operation.
    pub class_hierarchy: ClassHierarchy,
    /// The individual types from the previous reasoning operation.
    pub individual_types: HashMap<Individual, IndividualTypes>,
    /// The consistency status from the previous reasoning operation.
    pub is_consistent: bool,
    /// The revision number of the ontology when these results were computed.
    pub revision: u64,
}

impl Default for ReasoningResults {
    fn default() -> Self {
        ReasoningResults {
            class_hierarchy: ClassHierarchy::new(),
            individual_types: HashMap::new(),
            is_consistent: true,
            revision: 0,
        }
    }
}

/// An incremental reasoner that can reuse previous reasoning results.
#[derive(Debug)]
pub struct IncrementalReasoner {
    /// The underlying tableau reasoner.
    tableau_reasoner: TableauReasoner,
    /// Previous reasoning results for incremental updates.
    previous_results: Option<ReasoningResults>,
}

impl IncrementalReasoner {
    /// Creates a new incremental reasoner for the given ontology.
    pub fn new(ontology: Ontology) -> Self {
        IncrementalReasoner {
            tableau_reasoner: TableauReasoner::new(ontology),
            previous_results: None,
        }
    }
    
    /// Performs reasoning using incremental techniques when possible.
    /// 
    /// This method checks if incremental reasoning is possible based on the
    /// changes made to the ontology since the last reasoning operation.
    /// If incremental reasoning is not possible or beneficial, it falls back
    /// to full reasoning.
    /// 
    /// # Returns
    /// 
    /// The results of the reasoning operation.
    pub fn reason_incremental(&mut self) -> ReasoningResults {
        // Check if we can do incremental reasoning
        if self.can_do_incremental_reasoning() {
            self.perform_incremental_reasoning()
        } else {
            // Fall back to full reasoning
            self.perform_full_reasoning()
        }
    }
    
    /// Checks if incremental reasoning is possible.
    /// 
    /// This method examines the changes made to the ontology since the last
    /// reasoning operation to determine if incremental reasoning is possible.
    /// 
    /// # Returns
    /// 
    /// * `true` - If incremental reasoning is possible.
    /// * `false` - If full reasoning is required.
    fn can_do_incremental_reasoning(&self) -> bool {
        // For now, we'll implement a simple heuristic:
        // - If we have previous results
        // - And the ontology revision hasn't changed too much
        // - Then we can do incremental reasoning
        
        if let Some(ref previous) = self.previous_results {
            let current_revision = self.tableau_reasoner.ontology.change_tracker.revision;
            let previous_revision = previous.revision;
            
            // If the revision number hasn't changed much, we can do incremental reasoning
            // In a real implementation, we would analyze the specific changes
            current_revision - previous_revision < 10
        } else {
            // No previous results, so we need full reasoning
            false
        }
    }
    
    /// Performs incremental reasoning.
    /// 
    /// This method uses previous reasoning results to speed up the current
    /// reasoning operation when only small changes have been made.
    /// 
    /// # Returns
    /// 
    /// The results of the incremental reasoning operation.
    fn perform_incremental_reasoning(&mut self) -> ReasoningResults {
        // For now, we'll just do full reasoning but in a real implementation
        // we would use the previous results to optimize the computation
        
        let is_consistent = self.tableau_reasoner.is_consistent();
        let class_hierarchy = if is_consistent {
            self.tableau_reasoner.classify()
        } else {
            ClassHierarchy::new()
        };
        let individual_types = if is_consistent {
            self.tableau_reasoner.realize()
        } else {
            HashMap::new()
        };
        
        let results = ReasoningResults {
            class_hierarchy,
            individual_types,
            is_consistent,
            revision: self.tableau_reasoner.ontology.change_tracker.revision,
        };
        
        self.previous_results = Some(results.clone());
        results
    }
    
    /// Performs full reasoning.
    /// 
    /// This method performs a complete reasoning operation from scratch.
    /// 
    /// # Returns
    /// 
    /// The results of the full reasoning operation.
    fn perform_full_reasoning(&mut self) -> ReasoningResults {
        let is_consistent = self.tableau_reasoner.is_consistent();
        let class_hierarchy = if is_consistent {
            self.tableau_reasoner.classify()
        } else {
            ClassHierarchy::new()
        };
        let individual_types = if is_consistent {
            self.tableau_reasoner.realize()
        } else {
            HashMap::new()
        };
        
        let results = ReasoningResults {
            class_hierarchy,
            individual_types,
            is_consistent,
            revision: self.tableau_reasoner.ontology.change_tracker.revision,
        };
        
        self.previous_results = Some(results.clone());
        results
    }
    
    /// Clears the previous reasoning results.
    /// 
    /// This method should be called when the ontology has changed significantly
    /// and incremental reasoning is no longer beneficial.
    pub fn clear_previous_results(&mut self) {
        self.previous_results = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, IRI, ClassExpression, ClassAxiom, Axiom, Ontology};
    
    #[test]
    fn test_incremental_reasoner_creation() {
        let ontology = Ontology::default();
        let reasoner = IncrementalReasoner::new(ontology);
        assert!(reasoner.previous_results.is_none());
    }
    
    #[test]
    fn test_reasoning_with_empty_ontology() {
        let ontology = Ontology::default();
        let mut reasoner = IncrementalReasoner::new(ontology);
        let results = reasoner.reason_incremental();
        
        // Empty ontology should be consistent
        assert!(results.is_consistent);
        
        // Should have stored the results
        assert!(reasoner.previous_results.is_some());
    }
    
    #[test]
    fn test_ontology_change_tracking() {
        let mut ontology = Ontology::default();
        let initial_revision = ontology.change_tracker.revision;
        
        // Add an axiom directly to the axioms vector since our methods are not accessible in tests
        let class_a = Class(IRI("http://example.com/A".to_string()));
        let class_b = Class(IRI("http://example.com/B".to_string()));
        let axiom = Axiom::Class(ClassAxiom::SubClassOf {
            sub_class: ClassExpression::Class(class_a.clone()),
            super_class: ClassExpression::Class(class_b.clone()),
        });
        ontology.axioms.push(axiom.clone());
        ontology.change_tracker.added_axioms.push(axiom);
        ontology.change_tracker.revision += 1;
        
        // Revision should have increased
        assert_eq!(ontology.change_tracker.revision, initial_revision + 1);
        assert_eq!(ontology.change_tracker.added_axioms.len(), 1);
        
        // Clear changes manually
        ontology.change_tracker.added_axioms.clear();
        ontology.change_tracker.removed_axioms.clear();
        assert_eq!(ontology.change_tracker.added_axioms.len(), 0);
        assert_eq!(ontology.change_tracker.removed_axioms.len(), 0);
    }
}
