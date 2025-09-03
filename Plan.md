# Detailed Plan for owl2_rs

This document provides a detailed breakdown of the tasks outlined in `Project.md`. It serves as a checklist for the implementation of the `OWL2_rs` library.

---

### **Task 1: Core Data Structures**

-   [x] **Entities**
    -   [x] `struct Class(IRI)`
    -   [x] `struct Datatype(IRI)`
    -   [x] `struct ObjectProperty(IRI)`
    -   [x] `struct DataProperty(IRI)`
    -   [x] `struct AnnotationProperty(IRI)`
    -   [x] `struct NamedIndividual(IRI)`
    -   [x] `struct AnonymousIndividual(NodeID)`
-   [x] **Literals**
    -   [x] `struct Literal { value: String, datatype: Datatype, lang: Option<String> }`
-   [x] **Class Expressions**
    -   [x] `enum ClassExpression`
    -   [x] `Class(Class)`
    -   [x] `ObjectIntersectionOf(Vec<ClassExpression>)`
    -   [x] `ObjectUnionOf(Vec<ClassExpression>)`
    -   [x] `ObjectComplementOf(Box<ClassExpression>)`
    -   [x] `ObjectOneOf(Vec<Individual>)`
    -   [x] `ObjectSomeValuesFrom { property: ObjectPropertyExpression, filler: Box<ClassExpression> }`
    -   [x] `ObjectAllValuesFrom { property: ObjectPropertyExpression, filler: Box<ClassExpression> }`
    -   [x] `ObjectHasValue { property: ObjectPropertyExpression, value: Individual }`
    -   [x] `ObjectHasSelf(ObjectPropertyExpression)`
    -   [x] `ObjectMinCardinality { min: u32, property: ObjectPropertyExpression, filler: Option<Box<ClassExpression>> }`
    -   [x] `ObjectMaxCardinality { max: u32, property: ObjectPropertyExpression, filler: Option<Box<ClassExpression>> }`
    -   [x] `ObjectExactCardinality { cardinality: u32, property: ObjectPropertyExpression, filler: Option<Box<ClassExpression>> }`
-   [x] **Object Property Expressions**
    -   [x] `enum ObjectPropertyExpression`
    -   [x] `ObjectProperty(ObjectProperty)`
    -   [x] `InverseObjectProperty(ObjectProperty)`
    -   [x] `ObjectPropertyChain(Vec<ObjectPropertyExpression>)`
-   [x] **Data Ranges**
    -   [x] `enum DataRange`
    -   [x] `Datatype(Datatype)`
    -   [x] `DataIntersectionOf(Vec<DataRange>)`
    -   [x] `DataUnionOf(Vec<DataRange>)`
    -   [x] `DataComplementOf(Box<DataRange>)`
    -   [x] `DataOneOf(Vec<Literal>)`
    -   [x] `DatatypeRestriction { datatype: Datatype, restrictions: Vec<(IRI, Literal)> }`
-   [x] **Axioms**
    -   [x] **Class Axioms**
        -   [x] `enum ClassAxiom`
        -   [x] `SubClassOf { sub_class: ClassExpression, super_class: ClassExpression }`
        -   [x] `EquivalentClasses { classes: Vec<ClassExpression> }`
        -   [x] `DisjointClasses { classes: Vec<ClassExpression> }`
        -   [x] `DisjointUnion { class: Class, disjoint_classes: Vec<ClassExpression> }`
    -   [x] **Object Property Axioms**
        -   [x] `enum ObjectPropertyAxiom`
        -   [x] `SubObjectPropertyOf { sub_property: ObjectPropertyExpression, super_property: ObjectPropertyExpression }`
        -   [x] `EquivalentObjectProperties { properties: Vec<ObjectPropertyExpression> }`
        -   [x] `DisjointObjectProperties { properties: Vec<ObjectPropertyExpression> }`
        -   [x] `InverseObjectProperties { prop1: ObjectPropertyExpression, prop2: ObjectPropertyExpression }`
        -   [x] `ObjectPropertyDomain { property: ObjectPropertyExpression, domain: ClassExpression }`
        -   [x] `ObjectPropertyRange { property: ObjectPropertyExpression, range: ClassExpression }`
        -   [x] `FunctionalObjectProperty { property: ObjectPropertyExpression }`
        -   [x] `InverseFunctionalObjectProperty { property: ObjectPropertyExpression }`
        -   [x] `ReflexiveObjectProperty { property: ObjectPropertyExpression }`
        -   [x] `IrreflexiveObjectProperty { property: ObjectPropertyExpression }`
        -   [x] `SymmetricObjectProperty { property: ObjectPropertyExpression }`
        -   [x] `AsymmetricObjectProperty { property: ObjectPropertyExpression }`
        -   [x] `TransitiveObjectProperty { property: ObjectPropertyExpression }`
    -   [x] **Data Property Axioms**
        -   [x] `enum DataPropertyAxiom`
        -   [x] `SubDataPropertyOf { sub_property: DataProperty, super_property: DataProperty }`
        -   [x] `EquivalentDataProperties { properties: Vec<DataProperty> }`
        -   [x] `DisjointDataProperties { properties: Vec<DataProperty> }`
        -   [x] `DataPropertyDomain { property: DataProperty, domain: ClassExpression }`
        -   [x] `DataPropertyRange { property: DataProperty, range: DataRange }`
        -   [x] `FunctionalDataProperty { property: DataProperty }`
    -   [x] **Assertions**
        -   [x] `enum Assertion`
        -   [x] `SameIndividual { individuals: Vec<Individual> }`
        -   [x] `DifferentIndividuals { individuals: Vec<Individual> }`
        -   [x] `ClassAssertion { class: ClassExpression, individual: Individual }`
        -   [x] `ObjectPropertyAssertion { property: ObjectPropertyExpression, source: Individual, target: Individual }`
        -   [x] `DataPropertyAssertion { property: DataProperty, source: Individual, target: Literal }`
        -   [x] `NegativeObjectPropertyAssertion { property: ObjectPropertyExpression, source: Individual, target: Individual }`
        -   [x] `NegativeDataPropertyAssertion { property: DataProperty, source: Individual, target: Literal }`
-   [x] **Ontology Document**
    -   [x] `struct Ontology { direct_imports: Vec<IRI>, axioms: Vec<Axiom> }`

---

### **Task 2: OWL2 Parser (Functional-Style)**

-   [x] **Setup**
    -   [x] Choose and add a parser crate (`pest` or `nom`) to `Cargo.toml`.
    -   [x] Create `src/parser.rs` module.
-   [x] **Grammar/Parser Implementation**
    -   [x] Implement parsing for IRIs.
    -   [x] Implement parsing for Prefixes.
    -   [x] Implement parsing for Entities and Literals.
    -   [x] Implement parsing for Class Expressions.
    -   [x] Implement parsing for Property Expressions.
    -   [x] Implement parsing for all Axiom types.
    -   [x] Implement the top-level parser for an `Ontology` document.
-   [x] **Testing**
    -   [x] Create unit tests for each parser component.
    -   [x] Create an integration test that parses a simple, valid ontology file.

---

### **Task 3 & 4: Reasoner**

-   [x] **Core Structures**
    -   [x] `struct Tableau`
    -   [x] Data structures for the completion graph (nodes, edges, labels).
-   [x] **Tableau Rules**
    -   [x] Implement the conjunction (AND) rule.
    -   [x] Implement the disjunction (OR) rule.
    -   [x] Implement the existential (SOME) rule.
    -   [x] Implement the universal (ONLY) rule.
    -   [x] `... (and so on for all other rules)`
-   [x] **Reasoner Tasks**
    -   [x] **Consistency Check**
        -   [x] Implement the main loop for the tableau algorithm.
        -   [x] Implement clash detection (e.g., an individual is a member of A and not A).
    -   [x] **Classification**
        -   [x] Implement the algorithm to compute the subsumption hierarchy.
    -   [x] **Realization**
        -   [x] Implement the algorithm to find the most specific classes for each individual.
-   [x] **Testing**
    -   [x] Create tests for ontologies with known consistency/inconsistency.
    -   [x] Create tests for classification with known class hierarchies.

---

### **Task 5: Testing Framework - COMPLETED**

-   [x] **Unit Tests**
    -   [x] Go back through all data structures and ensure they have `#[test]` blocks covering their logic.
-   [x] **Integration Tests**
    -   [x] Find and download standard OWL2 conformance test suites.
    -   [x] Create a test runner that loads the test files, parses them, runs the reasoner, and compares the output with the expected results.

---

### **Task 6: Documentation and API Design - COMPLETED**

-   [x] **API Design**
    -   [x] Create a main `lib.rs` that exposes a clean public API.
    -   [x] `fn load_ontology(path: &Path) -> Result<Ontology, Error>`
    -   [x] `struct Reasoner { ontology: Ontology }`
    -   [x] `impl Reasoner { fn new(ontology: Ontology) -> Self; fn is_consistent(&self) -> bool; fn classify(&self) -> ClassHierarchy; }`
-   [x] **Documentation**
    -   [x] Write `//!` module-level documentation for `lib.rs` explaining the library's purpose.
    -   [x] Add `///` doc comments to all public structs, enums, and functions.
    -   [x] Create an `examples` directory with sample programs.
