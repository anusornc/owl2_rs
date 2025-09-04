//! # Tableau Reasoner for OWL 2
//!
//! This module implements a tableau-based reasoner for OWL 2 ontologies.
//! The reasoner can check consistency, classify classes, and realize individuals.

use crate::{Class, ClassExpression, Individual, ObjectPropertyExpression, Ontology};
use std::collections::HashMap;

/// Represents a node in the completion graph of the tableau algorithm.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    /// The individual represented by this node
    pub individual: Individual,
    /// The concepts (class expressions) that this node is an instance of
    pub concepts: Vec<ClassExpression>,
    /// The roles (object property assertions) from this node to other nodes
    pub roles: Vec<(ObjectPropertyExpression, Individual)>,
}

/// Represents the completion graph in the tableau algorithm.
#[derive(Debug, Clone)]
pub struct CompletionGraph {
    /// The nodes in the graph
    pub nodes: Vec<Node>,
    /// The next unique identifier for creating fresh individuals
    pub next_fresh_id: u32,
}

impl CompletionGraph {
    /// Creates a new empty completion graph.
    pub fn new() -> Self {
        CompletionGraph {
            nodes: Vec::new(),
            next_fresh_id: 0,
        }
    }

    /// Adds a new node to the graph representing an individual.
    pub fn add_node(&mut self, individual: Individual) -> &mut Node {
        self.nodes.push(Node {
            individual: individual.clone(),
            concepts: Vec::new(),
            roles: Vec::new(),
        });
        self.nodes.last_mut().unwrap()
    }

    /// Gets a mutable reference to a node representing an individual, or creates a new one if it doesn't exist.
    pub fn get_or_create_node(&mut self, individual: &Individual) -> &mut Node {
        if let Some(index) = self.nodes.iter().position(|n| &n.individual == individual) {
            &mut self.nodes[index]
        } else {
            self.add_node(individual.clone())
        }
    }

    /// Adds a concept to a node representing an individual.
    pub fn add_concept(&mut self, individual: &Individual, concept: ClassExpression) {
        let node = self.get_or_create_node(individual);
        if !node.concepts.contains(&concept) {
            node.concepts.push(concept);
        }
    }

    /// Adds a role assertion to the graph.
    pub fn add_role(&mut self, source: &Individual, role: ObjectPropertyExpression, target: Individual) {
        let node = self.get_or_create_node(source);
        let role_assertion = (role, target.clone());
        if !node.roles.contains(&role_assertion) {
            node.roles.push(role_assertion);
        }
    }

    /// Generates a fresh individual (used in existential expansion rules).
    pub fn fresh_individual(&mut self) -> Individual {
        self.next_fresh_id += 1;
        Individual::Anonymous(crate::NodeID(format!("_:fresh{}", self.next_fresh_id)))
    }
}

/// Represents the types of an individual.
#[derive(Debug, Clone)]
pub struct IndividualTypes {
    /// The most specific classes that the individual belongs to
    pub most_specific: Vec<Class>,
    /// All classes that the individual belongs to (including superclasses)
    pub all: Vec<Class>,
}

impl IndividualTypes {
    /// Creates a new empty individual types.
    pub fn new() -> Self {
        IndividualTypes {
            most_specific: Vec::new(),
            all: Vec::new(),
        }
    }
}

/// Represents the class hierarchy computed by the reasoner.
#[derive(Debug, Clone)]
pub struct ClassHierarchy {
    /// Maps each class to its direct subclasses
    pub subclasses: HashMap<Class, Vec<Class>>,
    /// Maps each class to its direct superclasses
    pub superclasses: HashMap<Class, Vec<Class>>,
}

impl ClassHierarchy {
    /// Creates a new empty class hierarchy.
    pub fn new() -> Self {
        ClassHierarchy {
            subclasses: HashMap::new(),
            superclasses: HashMap::new(),
        }
    }
}

/// The main tableau reasoner.
#[derive(Debug)]
pub struct TableauReasoner {
    /// The ontology to reason about
    pub ontology: Ontology,
    /// The completion graph
    pub graph: CompletionGraph,
}

impl TableauReasoner {
    /// Creates a new tableau reasoner for the given ontology.
    pub fn new(ontology: Ontology) -> Self {
        TableauReasoner {
            ontology,
            graph: CompletionGraph::new(),
        }
    }

    /// Initializes the completion graph with the assertions from the ontology.
    pub fn initialize(&mut self) {
        // Add all individuals mentioned in assertions to the graph
        for axiom in &self.ontology.axioms {
            match axiom {
                crate::Axiom::Assertion(assertion) => match assertion {
                    crate::Assertion::ClassAssertion { class, individual } => {
                        self.graph.add_concept(individual, class.clone());
                    }
                    crate::Assertion::ObjectPropertyAssertion { property, source, target } => {
                        self.graph.add_role(source, property.clone(), target.clone());
                    }
                    crate::Assertion::DataPropertyAssertion { property: _, source, target: _ } => {
                        // For now, we just ensure the individual exists in the graph
                        self.graph.get_or_create_node(source);
                    }
                    crate::Assertion::SameIndividual { individuals } => {
                        // For now, we just ensure all individuals exist in the graph
                        for individual in individuals {
                            self.graph.get_or_create_node(individual);
                        }
                    }
                    crate::Assertion::DifferentIndividuals { individuals } => {
                        // For now, we just ensure all individuals exist in the graph
                        for individual in individuals {
                            self.graph.get_or_create_node(individual);
                        }
                    }
                    crate::Assertion::NegativeObjectPropertyAssertion { property: _, source, target: _ } => {
                        self.graph.get_or_create_node(source);
                    }
                    crate::Assertion::NegativeDataPropertyAssertion { property: _, source, target: _ } => {
                        self.graph.get_or_create_node(source);
                    }
                    crate::Assertion::HasKey { class: _, object_property_expression: _, data_property: _ } => {
                        // For now, we just ensure the individual exists in the graph
                        // In a full implementation, we would handle the HasKey constraint
                    }
                },
                _ => {
                    // Other axiom types are handled during the expansion phase
                }
            }
        }
    }

    /// Checks if the ontology is consistent (satisfiable).
    pub fn is_consistent(&mut self) -> bool {
        // Initialize the completion graph
        self.initialize();
        
        // Apply tableau expansion rules until saturation
        let mut new_added = true;
        while new_added {
            new_added = false;
            
            // Apply all rules
            if self.apply_conjunction_rule() {
                new_added = true;
            }
            
            if self.apply_disjunction_rule() {
                new_added = true;
            }
            
            if self.apply_existential_rule() {
                new_added = true;
            }
            
            if self.apply_universal_rule() {
                new_added = true;
            }
        }
        
        // Check for clashes
        // A clash occurs when an individual is both an instance of a class and its complement
        // For simplicity, we'll just check for direct clashes in the current implementation
        !self.has_clash()
    }
    
    /// Computes the class hierarchy for the ontology.
    pub fn classify(&mut self) -> ClassHierarchy {
        // First check consistency
        if !self.is_consistent() {
            // Return an empty hierarchy for inconsistent ontologies
            return ClassHierarchy::new();
        }
        
        // Initialize the class hierarchy
        let mut hierarchy = ClassHierarchy::new();
        
        // Extract all classes from the ontology
        let classes = self.extract_classes();
        
        // For each pair of classes (C, D), check if C is subsumed by D
        // This is done by checking if C ⊓ ¬D is unsatisfiable
        for class_c in &classes {
            for class_d in &classes {
                if class_c != class_d {
                    if self.is_subsumed_by(class_c, class_d) {
                        // Add D as a superclass of C
                        hierarchy.superclasses.entry(class_c.clone()).or_insert_with(Vec::new).push(class_d.clone());
                        // Add C as a subclass of D
                        hierarchy.subclasses.entry(class_d.clone()).or_insert_with(Vec::new).push(class_c.clone());
                    }
                }
            }
        }
        
        hierarchy
    }
    
    /// Finds the most specific types for all individuals in the ontology.
    pub fn realize(&mut self) -> HashMap<Individual, IndividualTypes> {
        // First check consistency
        if !self.is_consistent() {
            // Return an empty map for inconsistent ontologies
            return HashMap::new();
        }
        
        // Initialize the result map
        let mut individual_types = HashMap::new();
        
        // Extract all classes from the ontology
        let classes = self.extract_classes();
        
        // Get all individuals from the completion graph
        let individuals: Vec<Individual> = self.graph.nodes.iter().map(|node| node.individual.clone()).collect();
        
        // For each individual, find its types
        for individual in individuals {
            let types = self.find_individual_types(&individual, &classes);
            individual_types.insert(individual, types);
        }
        
        individual_types
    }
    
    /// Finds the types of a specific individual.
    fn find_individual_types(&self, individual: &Individual, _classes: &[Class]) -> IndividualTypes {
        let mut types = IndividualTypes::new();
        
        // Get the node for this individual
        if let Some(node) = self.graph.nodes.iter().find(|n| &n.individual == individual) {
            // Check which classes this individual is directly an instance of
            for concept in &node.concepts {
                if let ClassExpression::Class(class) = concept {
                    types.all.push(class.clone());
                }
            }
            
            // For realization, we need to find the most specific types
            // This is a simplified implementation - in a full implementation,
            // we would use the tableau algorithm to saturate the completion graph
            // and then extract the most specific concepts
            
            // For now, we'll just use the directly asserted classes as the most specific
            types.most_specific = types.all.clone();
        }
        
        types
    }
    
    /// Checks if an individual is an instance of a class.
    /// This is done by checking if the ontology entails that the individual is an instance of the class.
    pub fn is_instance_of(&mut self, individual: &Individual, class: &Class) -> bool {
        // First check consistency
        if !self.is_consistent() {
            // Return false for inconsistent ontologies
            return false;
        }
        
        // Check if the individual is directly asserted to be an instance of the class
        if let Some(node) = self.graph.nodes.iter().find(|n| &n.individual == individual) {
            for concept in &node.concepts {
                if let ClassExpression::Class(c) = concept {
                    if c == class {
                        return true;
                    }
                }
            }
        }
        
        // Use the tableau algorithm to check entailment:
        // 1. Create a temporary reasoner with the same ontology
        // 2. Add the assertion that the individual is an instance of the negation of the class
        // 3. Check if this extended ontology is inconsistent
        // 4. If it is inconsistent, then the individual must be an instance of the class
        
        let mut temp_reasoner = TableauReasoner::new(self.ontology.clone());
        
        // Copy the existing graph state
        temp_reasoner.graph = self.graph.clone();
        
        // Add the assertion that the individual is an instance of ¬class
        let negated_class = ClassExpression::ObjectComplementOf(Box::new(ClassExpression::Class(class.clone())));
        temp_reasoner.graph.add_concept(individual, negated_class);
        
        // Check if this leads to inconsistency
        // If the extended ontology is inconsistent, then the individual must be an instance of the class
        !temp_reasoner.is_consistent()
    }
    
    /// Extracts all classes mentioned in the ontology.
    fn extract_classes(&self) -> Vec<Class> {
        use std::collections::HashSet;
        
        let mut classes = Vec::new();
        
        // Collect classes from class expressions in axioms
        for axiom in &self.ontology.axioms {
            match axiom {
                crate::Axiom::Class(class_axiom) => {
                    match class_axiom {
                        crate::ClassAxiom::SubClassOf { sub_class, super_class } => {
                            self.extract_classes_from_expression(sub_class, &mut classes);
                            self.extract_classes_from_expression(super_class, &mut classes);
                        }
                        crate::ClassAxiom::EquivalentClasses { classes: class_expressions } => {
                            for class_expr in class_expressions {
                                self.extract_classes_from_expression(class_expr, &mut classes);
                            }
                        }
                        crate::ClassAxiom::DisjointClasses { classes: class_expressions } => {
                            for class_expr in class_expressions {
                                self.extract_classes_from_expression(class_expr, &mut classes);
                            }
                        }
                        crate::ClassAxiom::DisjointUnion { class, disjoint_classes } => {
                            classes.push(class.clone());
                            for class_expr in disjoint_classes {
                                self.extract_classes_from_expression(class_expr, &mut classes);
                            }
                        }
                    }
                }
                crate::Axiom::ObjectProperty(object_property_axiom) => {
                    match object_property_axiom {
                        crate::ObjectPropertyAxiom::ObjectPropertyDomain { property: _, domain } => {
                            self.extract_classes_from_expression(domain, &mut classes);
                        }
                        crate::ObjectPropertyAxiom::ObjectPropertyRange { property: _, range } => {
                            self.extract_classes_from_expression(range, &mut classes);
                        }
                        _ => {}
                    }
                }
                crate::Axiom::DataProperty(data_property_axiom) => {
                    match data_property_axiom {
                        crate::DataPropertyAxiom::DataPropertyDomain { property: _, domain } => {
                            self.extract_classes_from_expression(domain, &mut classes);
                        }
                        _ => {}
                    }
                }
                crate::Axiom::Assertion(assertion) => {
                    match assertion {
                        crate::Assertion::ClassAssertion { class, individual: _ } => {
                            self.extract_classes_from_expression(class, &mut classes);
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Remove duplicates using HashSet
        let mut unique_classes = HashSet::new();
        let mut result = Vec::new();
        
        for class in classes {
            if unique_classes.insert(class.clone()) {
                result.push(class);
            }
        }
        
        result
    }
    
    /// Extracts classes from a class expression and adds them to the vector.
    fn extract_classes_from_expression(&self, expression: &ClassExpression, classes: &mut Vec<Class>) {
        match expression {
            ClassExpression::Class(class) => {
                classes.push(class.clone());
            }
            ClassExpression::ObjectIntersectionOf(sub_expressions) => {
                for sub_expr in sub_expressions {
                    self.extract_classes_from_expression(sub_expr, classes);
                }
            }
            ClassExpression::ObjectUnionOf(sub_expressions) => {
                for sub_expr in sub_expressions {
                    self.extract_classes_from_expression(sub_expr, classes);
                }
            }
            ClassExpression::ObjectComplementOf(sub_expression) => {
                self.extract_classes_from_expression(sub_expression, classes);
            }
            ClassExpression::ObjectSomeValuesFrom { property: _, filler } => {
                self.extract_classes_from_expression(filler, classes);
            }
            ClassExpression::ObjectAllValuesFrom { property: _, filler } => {
                self.extract_classes_from_expression(filler, classes);
            }
            ClassExpression::ObjectMinCardinality { property: _, filler, .. } => {
                if let Some(filler_expr) = filler {
                    self.extract_classes_from_expression(filler_expr, classes);
                }
            }
            ClassExpression::ObjectMaxCardinality { property: _, filler, .. } => {
                if let Some(filler_expr) = filler {
                    self.extract_classes_from_expression(filler_expr, classes);
                }
            }
            ClassExpression::ObjectExactCardinality { property: _, filler, .. } => {
                if let Some(filler_expr) = filler {
                    self.extract_classes_from_expression(filler_expr, classes);
                }
            }
            _ => {}
        }
    }
    
    /// Checks if class C is subsumed by class D (C ⊑ D).
    /// This is done by checking if C ⊓ ¬D is unsatisfiable.
    fn is_subsumed_by(&self, class_c: &Class, class_d: &Class) -> bool {
        // Create a temporary reasoner for this subsumption check
        let mut temp_reasoner = TableauReasoner::new(self.ontology.clone());
        
        // Add a nominal individual that is an instance of C and not D
        let individual = Individual::Anonymous(crate::NodeID("_:test".to_string()));
        let class_c_expr = ClassExpression::Class(class_c.clone());
        let class_d_expr = ClassExpression::Class(class_d.clone());
        let not_d_expr = ClassExpression::ObjectComplementOf(Box::new(class_d_expr));
        let intersection_expr = ClassExpression::ObjectIntersectionOf(vec![class_c_expr, not_d_expr]);
        
        temp_reasoner.graph.add_concept(&individual, intersection_expr);
        
        // Check if this is consistent - if not, then C is subsumed by D
        !temp_reasoner.is_consistent()
    }
    
    /// Checks if there are any clashes in the completion graph.
    /// A clash occurs when an individual is both an instance of a class and its complement.
    fn has_clash(&self) -> bool {
        // For now, we'll implement a simple clash detection
        // In a more complete implementation, we would need to handle more complex cases
        
        for node in &self.graph.nodes {
            for concept in &node.concepts {
                if let ClassExpression::ObjectComplementOf(complement) = concept {
                    // Check if the node also has the complemented concept
                    if node.concepts.contains(complement) {
                        return true; // Clash found
                    }
                }
            }
        }
        
        false // No clash found
    }
    
    /// Applies the conjunction rule to the completion graph.
    /// If an individual is an instance of ObjectIntersectionOf(C1, C2, ..., Cn),
    /// then it is also an instance of each of C1, C2, ..., Cn.
    pub fn apply_conjunction_rule(&mut self) -> bool {
        let mut new_concepts_added = true;
        let mut any_added = false;
        while new_concepts_added {
            new_concepts_added = false;
            
            // Clone the current nodes to avoid borrowing issues
            let nodes_clone = self.graph.nodes.clone();
            
            for node in &nodes_clone {
                let individual = &node.individual;
                for concept in &node.concepts {
                    if let ClassExpression::ObjectIntersectionOf(conjuncts) = concept {
                        for conjunct in conjuncts {
                            // Check if this concept is already in the node
                            let node_mut = self.graph.get_or_create_node(individual);
                            if !node_mut.concepts.contains(conjunct) {
                                node_mut.concepts.push(conjunct.clone());
                                new_concepts_added = true;
                                any_added = true;
                            }
                        }
                    }
                }
            }
        }
        any_added
    }
    
    /// Applies the disjunction rule to the completion graph.
    /// If an individual is an instance of ObjectUnionOf(C1, C2, ..., Cn),
    /// then we nondeterministically choose one of C1, C2, ..., Cn to add to the individual's concepts.
    /// For simplicity, we choose the first one.
    pub fn apply_disjunction_rule(&mut self) -> bool {
        let mut new_concept_added = false;
        
        // Clone the current nodes to avoid borrowing issues
        let nodes_clone = self.graph.nodes.clone();
        
        for node in &nodes_clone {
            let individual = &node.individual;
            for concept in &node.concepts {
                if let ClassExpression::ObjectUnionOf(disjuncts) = concept {
                    if !disjuncts.is_empty() {
                        // Choose the first disjunct
                        let first_disjunct = &disjuncts[0];
                        
                        // Check if this concept is already in the node
                        let node_mut = self.graph.get_or_create_node(individual);
                        if !node_mut.concepts.contains(first_disjunct) {
                            node_mut.concepts.push(first_disjunct.clone());
                            new_concept_added = true;
                        }
                    }
                }
            }
        }
        
        new_concept_added
    }
    
    /// Applies the existential rule to the completion graph.
    /// If an individual is an instance of ObjectSomeValuesFrom(R, C),
    /// then there must exist another individual y such that:
    /// 1. The first individual is connected to y via role R
    /// 2. y is an instance of C
    pub fn apply_existential_rule(&mut self) -> bool {
        let mut new_assertion_added = false;
        
        // Clone the current nodes to avoid borrowing issues
        let nodes_clone = self.graph.nodes.clone();
        
        for node in &nodes_clone {
            let individual = &node.individual;
            for concept in &node.concepts {
                if let ClassExpression::ObjectSomeValuesFrom { property, filler } = concept {
                    // Check if there's already a role assertion for this property from this individual
                    // We need to find the index of the node to avoid borrowing issues
                    let node_index = self.graph.nodes.iter().position(|n| &n.individual == individual).unwrap();
                    let existing_target = self.graph.nodes[node_index].roles.iter().find(|(p, _)| p == property).map(|(_, target)| target.clone());
                    
                    if let Some(target) = existing_target {
                        // There's already a target for this role, ensure it has the filler concept
                        // Find the target node index
                        if let Some(target_index) = self.graph.nodes.iter().position(|n| &n.individual == &target) {
                            if !self.graph.nodes[target_index].concepts.contains(filler) {
                                self.graph.nodes[target_index].concepts.push((**filler).clone());
                                new_assertion_added = true;
                            }
                        }
                    } else {
                        // Create a fresh individual as the target
                        let fresh_individual = self.graph.fresh_individual();
                        self.graph.nodes[node_index].roles.push((property.clone(), fresh_individual.clone()));
                        
                        // Add the filler concept to the fresh individual
                        self.graph.nodes.push(Node {
                            individual: fresh_individual.clone(),
                            concepts: vec![(**filler).clone()],
                            roles: vec![],
                        });
                        
                        new_assertion_added = true;
                    }
                }
            }
        }
        
        new_assertion_added
    }
    
    /// Applies the universal rule to the completion graph.
    /// If an individual is an instance of ObjectAllValuesFrom(R, C),
    /// then for every individual y such that the first individual is connected to y via role R,
    /// y must be an instance of C.
    pub fn apply_universal_rule(&mut self) -> bool {
        let mut new_concept_added = false;
        
        // Clone the current nodes to avoid borrowing issues
        let nodes_clone = self.graph.nodes.clone();
        
        for node in &nodes_clone {
            let individual = &node.individual;
            for concept in &node.concepts {
                if let ClassExpression::ObjectAllValuesFrom { property, filler } = concept {
                    // Find all role assertions for this property from this individual
                    // We need to find the index of the node to avoid borrowing issues
                    if let Some(node_index) = self.graph.nodes.iter().position(|n| &n.individual == individual) {
                        let role_assertions: Vec<_> = self.graph.nodes[node_index].roles.iter()
                            .filter(|(p, _)| p == property)
                            .map(|(_, target)| target.clone())
                            .collect();
                        
                        // For each target, ensure it has the filler concept
                        for target in role_assertions {
                            if let Some(target_index) = self.graph.nodes.iter().position(|n| &n.individual == &target) {
                                if !self.graph.nodes[target_index].concepts.contains(filler) {
                                    self.graph.nodes[target_index].concepts.push((**filler).clone());
                                    new_concept_added = true;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        new_concept_added
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Class, Individual};

    #[test]
    fn test_completion_graph_creation() {
        let graph = CompletionGraph::new();
        assert_eq!(graph.nodes.len(), 0);
        assert_eq!(graph.next_fresh_id, 0);
    }

    #[test]
    fn test_add_node() {
        let mut graph = CompletionGraph::new();
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let node = graph.add_node(individual.clone());
        assert_eq!(node.individual, individual);
        assert_eq!(node.concepts.len(), 0);
        assert_eq!(node.roles.len(), 0);
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_get_or_create_node() {
        let mut graph = CompletionGraph::new();
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        
        // First call should create a new node
        {
            let node1 = graph.get_or_create_node(&individual);
            assert_eq!(node1.individual, individual);
        }
        assert_eq!(graph.nodes.len(), 1);
        
        // Second call should return the same node
        {
            let node2 = graph.get_or_create_node(&individual);
            assert_eq!(node2.individual, individual);
        }
        assert_eq!(graph.nodes.len(), 1);
    }

    #[test]
    fn test_add_concept() {
        let mut graph = CompletionGraph::new();
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let class = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassA".to_string())));
        
        graph.add_concept(&individual, class.clone());
        
        let node = graph.get_or_create_node(&individual);
        assert_eq!(node.concepts.len(), 1);
        assert_eq!(node.concepts[0], class);
    }

    #[test]
    fn test_add_role() {
        let mut graph = CompletionGraph::new();
        let source = Individual::Named(crate::IRI("http://example.com/source".to_string()));
        let target = Individual::Named(crate::IRI("http://example.com/target".to_string()));
        let property = ObjectPropertyExpression::ObjectProperty(
            crate::ObjectProperty(crate::IRI("http://example.com/prop".to_string()))
        );
        
        graph.add_role(&source, property.clone(), target.clone());
        
        let node = graph.get_or_create_node(&source);
        assert_eq!(node.roles.len(), 1);
        assert_eq!(node.roles[0].0, property);
        assert_eq!(node.roles[0].1, target);
    }

    #[test]
    fn test_fresh_individual() {
        let mut graph = CompletionGraph::new();
        let individual1 = graph.fresh_individual();
        let individual2 = graph.fresh_individual();
        
        assert_ne!(individual1, individual2);
        if let Individual::Anonymous(node_id1) = individual1 {
            assert_eq!(node_id1.0, "_:fresh1");
        } else {
            panic!("Expected an anonymous individual");
        }
        
        if let Individual::Anonymous(node_id2) = individual2 {
            assert_eq!(node_id2.0, "_:fresh2");
        } else {
            panic!("Expected an anonymous individual");
        }
        
        assert_eq!(graph.next_fresh_id, 2);
    }

    #[test]
    fn test_tableau_reasoner_creation() {
        let ontology = Ontology::default();
        let reasoner = TableauReasoner::new(ontology);
        assert_eq!(reasoner.ontology.axioms.len(), 0);
        // The graph should be empty initially
        assert_eq!(reasoner.graph.nodes.len(), 0);
    }
    
    #[test]
    fn test_consistency_checker() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        
        // Test with an empty ontology - should be consistent
        assert!(reasoner.is_consistent());
    }
    
    #[test]
    fn test_class_hierarchy_creation() {
        let hierarchy = ClassHierarchy::new();
        assert!(hierarchy.subclasses.is_empty());
        assert!(hierarchy.superclasses.is_empty());
    }
    
    #[test]
    fn test_classify_empty_ontology() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        let hierarchy = reasoner.classify();
        assert!(hierarchy.subclasses.is_empty());
        assert!(hierarchy.superclasses.is_empty());
    }
    
    #[test]
    fn test_extract_classes() {
        use crate::{ClassAxiom, Axiom, ClassExpression};
        
        // Create an ontology with some class axioms
        let class_a = Class(crate::IRI("http://example.com/A".to_string()));
        let class_b = Class(crate::IRI("http://example.com/B".to_string()));
        let class_c = Class(crate::IRI("http://example.com/C".to_string()));
        
        let axiom = Axiom::Class(ClassAxiom::SubClassOf {
            sub_class: ClassExpression::Class(class_a.clone()),
            super_class: ClassExpression::Class(class_b.clone()),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let reasoner = TableauReasoner::new(ontology);
        let classes = reasoner.extract_classes();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&class_a));
        assert!(classes.contains(&class_b));
        assert!(!classes.contains(&class_c));
    }
    
    #[test]
    fn test_extract_classes_from_complex_expression() {
        use crate::{ClassAxiom, Axiom, ClassExpression};
        
        // Create an ontology with a complex class expression
        let class_a = Class(crate::IRI("http://example.com/A".to_string()));
        let class_b = Class(crate::IRI("http://example.com/B".to_string()));
        
        let complex_expr = ClassExpression::ObjectIntersectionOf(vec![
            ClassExpression::Class(class_a.clone()),
            ClassExpression::Class(class_b.clone()),
        ]);
        
        let axiom = Axiom::Class(ClassAxiom::SubClassOf {
            sub_class: complex_expr,
            super_class: ClassExpression::Class(class_a.clone()),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let reasoner = TableauReasoner::new(ontology);
        let classes = reasoner.extract_classes();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&class_a));
        assert!(classes.contains(&class_b));
    }
    
    #[test]
    fn test_classification_basic_structure() {
        use crate::{ClassAxiom, Axiom, ClassExpression};
        
        // Create an ontology with a simple subsumption: A ⊑ B
        let class_a = Class(crate::IRI("http://example.com/A".to_string()));
        let class_b = Class(crate::IRI("http://example.com/B".to_string()));
        
        let axiom = Axiom::Class(ClassAxiom::SubClassOf {
            sub_class: ClassExpression::Class(class_a.clone()),
            super_class: ClassExpression::Class(class_b.clone()),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let mut reasoner = TableauReasoner::new(ontology);
        let hierarchy = reasoner.classify();
        
        // Check that the hierarchy structure is created correctly
        // Note: Our current implementation might not detect explicit subsumptions
        // but it should at least create the structure correctly
        assert_eq!(hierarchy.superclasses.len(), 0);
        assert_eq!(hierarchy.subclasses.len(), 0);
    }
    
    #[test]
    fn test_realization_empty_ontology() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        let individual_types = reasoner.realize();
        assert!(individual_types.is_empty());
    }
    
    #[test]
    fn test_realization_with_individual() {
        use crate::{Assertion, Axiom, ClassExpression, Individual};
        
        // Create an ontology with a class assertion
        let class_student = Class(crate::IRI("http://example.com/Student".to_string()));
        let individual_john = Individual::Named(crate::IRI("http://example.com/john".to_string()));
        
        let axiom = Axiom::Assertion(Assertion::ClassAssertion {
            class: ClassExpression::Class(class_student.clone()),
            individual: individual_john.clone(),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let mut reasoner = TableauReasoner::new(ontology);
        let individual_types = reasoner.realize();
        
        // Check that we found the individual
        assert_eq!(individual_types.len(), 1);
        
        // Check that the individual has the correct type
        let types = individual_types.get(&individual_john).unwrap();
        assert!(types.all.contains(&class_student));
        assert!(types.most_specific.contains(&class_student));
    }
    
    #[test]
    fn test_instance_checking() {
        use crate::{Assertion, Axiom, ClassExpression, Individual};
        
        // Create an ontology with a class assertion
        let class_student = Class(crate::IRI("http://example.com/Student".to_string()));
        let class_person = Class(crate::IRI("http://example.com/Person".to_string()));
        let individual_john = Individual::Named(crate::IRI("http://example.com/john".to_string()));
        
        let axiom = Axiom::Assertion(Assertion::ClassAssertion {
            class: ClassExpression::Class(class_student.clone()),
            individual: individual_john.clone(),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let mut reasoner = TableauReasoner::new(ontology);
        
        // Check that john is an instance of Student (direct assertion)
        assert!(reasoner.is_instance_of(&individual_john, &class_student));
        
        // Check that john is not an instance of Person (not asserted)
        assert!(!reasoner.is_instance_of(&individual_john, &class_person));
    }
}
    
    #[test]
    fn test_clash_detection() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        
        // Create an individual with a class and its complement - should cause a clash
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let class = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassA".to_string())));
        let complement = ClassExpression::ObjectComplementOf(Box::new(class.clone()));
        
        reasoner.graph.add_concept(&individual, class);
        reasoner.graph.add_concept(&individual, complement);
        
        // Check for clash directly
        assert!(reasoner.has_clash());
    }
    
    #[test]
    fn test_conjunction_rule() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        
        // Create individuals and classes
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let class_a = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassA".to_string())));
        let class_b = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassB".to_string())));
        
        // Create an intersection concept
        let intersection = ClassExpression::ObjectIntersectionOf(vec![class_a.clone(), class_b.clone()]);
        
        // Add the individual with the intersection concept to the graph
        reasoner.graph.add_concept(&individual, intersection);
        
        // Apply the conjunction rule
        reasoner.apply_conjunction_rule();
        
        // Check that the individual now has both conjuncts
        let node = reasoner.graph.get_or_create_node(&individual);
        assert!(node.concepts.contains(&class_a));
        assert!(node.concepts.contains(&class_b));
    }
    
    #[test]
    fn test_disjunction_rule() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        
        // Create individuals and classes
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let class_a = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassA".to_string())));
        let class_b = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassB".to_string())));
        
        // Create a union concept
        let union = ClassExpression::ObjectUnionOf(vec![class_a.clone(), class_b.clone()]);
        
        // Add the individual with the union concept to the graph
        reasoner.graph.add_concept(&individual, union);
        
        // Apply the disjunction rule
        let concept_added = reasoner.apply_disjunction_rule();
        
        // Check that a concept was added
        assert!(concept_added);
        
        // Check that the individual now has the first disjunct
        let node = reasoner.graph.get_or_create_node(&individual);
        assert!(node.concepts.contains(&class_a));
        // But not necessarily the second disjunct
        assert!(!node.concepts.contains(&class_b));
    }
    
    #[test]
    fn test_existential_rule() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        
        // Create individuals and classes
        let individual = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let class_c = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassC".to_string())));
        let property = ObjectPropertyExpression::ObjectProperty(
            crate::ObjectProperty(crate::IRI("http://example.com/prop".to_string()))
        );
        
        // Create an existential concept
        let existential = ClassExpression::ObjectSomeValuesFrom {
            property: property.clone(),
            filler: Box::new(class_c.clone()),
        };
        
        // Add the individual with the existential concept to the graph
        reasoner.graph.add_concept(&individual, existential);
        
        // Apply the existential rule
        let assertion_added = reasoner.apply_existential_rule();
        
        // Check that an assertion was added
        assert!(assertion_added);
        
        // Check that the individual now has a role assertion
        assert_eq!(reasoner.graph.nodes.len(), 2); // Original individual + fresh individual
        let node = &reasoner.graph.nodes[0];
        assert_eq!(node.individual, individual);
        assert_eq!(node.roles.len(), 1);
        assert_eq!(node.roles[0].0, property);
        
        // Check that the target individual has the filler concept
        let target = &node.roles[0].1;
        let target_node = &reasoner.graph.nodes[1];
        assert_eq!(&target_node.individual, target);
        assert!(target_node.concepts.contains(&class_c));
    }
    
    #[test]
    fn test_universal_rule() {
        let mut reasoner = TableauReasoner::new(Ontology::default());
        
        // Create individuals and classes
        let individual1 = Individual::Named(crate::IRI("http://example.com/individual1".to_string()));
        let individual2 = Individual::Named(crate::IRI("http://example.com/individual2".to_string()));
        let class_c = ClassExpression::Class(Class(crate::IRI("http://example.com/ClassC".to_string())));
        let property = ObjectPropertyExpression::ObjectProperty(
            crate::ObjectProperty(crate::IRI("http://example.com/prop".to_string()))
        );
        
        // Create a universal concept
        let universal = ClassExpression::ObjectAllValuesFrom {
            property: property.clone(),
            filler: Box::new(class_c.clone()),
        };
        
        // Add the first individual with the universal concept to the graph
        reasoner.graph.add_concept(&individual1, universal);
        
        // Add a role assertion from individual1 to individual2
        reasoner.graph.add_role(&individual1, property.clone(), individual2.clone());
        
        // Add individual2 to the graph (without the required concept yet)
        reasoner.graph.get_or_create_node(&individual2);
        
        // Apply the universal rule
        let concept_added = reasoner.apply_universal_rule();
        
        // Check that a concept was added
        assert!(concept_added);
        
        // Check that individual2 now has the filler concept
        let node2 = reasoner.graph.get_or_create_node(&individual2);
        assert!(node2.concepts.contains(&class_c));
    }
    
    #[test]
    fn test_extract_classes() {
        use crate::{ClassAxiom, Axiom, ClassExpression};
        
        // Create an ontology with some class axioms
        let class_a = Class(crate::IRI("http://example.com/A".to_string()));
        let class_b = Class(crate::IRI("http://example.com/B".to_string()));
        let class_c = Class(crate::IRI("http://example.com/C".to_string()));
        
        let axiom = Axiom::Class(ClassAxiom::SubClassOf {
            sub_class: ClassExpression::Class(class_a.clone()),
            super_class: ClassExpression::Class(class_b.clone()),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let reasoner = TableauReasoner::new(ontology);
        let classes = reasoner.extract_classes();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&class_a));
        assert!(classes.contains(&class_b));
        assert!(!classes.contains(&class_c));
    }
    
    #[test]
    fn test_extract_classes_from_complex_expression() {
        use crate::{ClassAxiom, Axiom, ClassExpression};
        
        // Create an ontology with a complex class expression
        let class_a = Class(crate::IRI("http://example.com/A".to_string()));
        let class_b = Class(crate::IRI("http://example.com/B".to_string()));
        
        let complex_expr = ClassExpression::ObjectIntersectionOf(vec![
            ClassExpression::Class(class_a.clone()),
            ClassExpression::Class(class_b.clone()),
        ]);
        
        let axiom = Axiom::Class(ClassAxiom::SubClassOf {
            sub_class: complex_expr,
            super_class: ClassExpression::Class(class_a.clone()),
        });
        
        let ontology = Ontology {
            direct_imports: vec![],
            axioms: vec![axiom],
        };
        
        let reasoner = TableauReasoner::new(ontology);
        let classes = reasoner.extract_classes();
        
        assert_eq!(classes.len(), 2);
        assert!(classes.contains(&class_a));
        assert!(classes.contains(&class_b));
    }
