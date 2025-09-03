# API Reference for owl2_rs

This document provides detailed documentation for the public API of the owl2_rs library.

## Table of Contents

1. [Main Modules](#main-modules)
2. [Loading Ontologies](#loading-ontologies)
3. [Reasoning](#reasoning)
4. [Core Data Structures](#core-data-structures)

## Main Modules

The owl2_rs library is organized into several modules:

- `api`: The main public API for the library
- `parser`: The OWL 2 parser implementation
- `reasoner`: The tableau-based reasoner implementation

For most use cases, you'll only need to interact with the `api` module.

## Loading Ontologies

### `load_ontology`

```rust
pub fn load_ontology(input: &str) -> Result<Ontology, Owl2RsError>
```

Loads an ontology from a string in OWL 2 Functional-Style Syntax.

**Arguments:**
- `input`: A string containing the ontology in OWL 2 Functional-Style Syntax.

**Returns:**
- `Ok(Ontology)`: The parsed ontology.
- `Err(Owl2RsError)`: An error if parsing fails.

**Example:**
```rust
use owl2_rs::api::load_ontology;

let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;

let ontology = load_ontology(ontology_str)?;
```

### `load_ontology_from_file`

```rust
pub fn load_ontology_from_file(path: &Path) -> Result<Ontology, Owl2RsError>
```

Loads an ontology from a file containing OWL 2 Functional-Style Syntax.

**Arguments:**
- `path`: The path to the file containing the ontology.

**Returns:**
- `Ok(Ontology)`: The parsed ontology.
- `Err(Owl2RsError)`: An error if reading the file or parsing fails.

**Example:**
```rust
use owl2_rs::api::load_ontology_from_file;
use std::path::Path;

let ontology = load_ontology_from_file(Path::new("ontology.ofn"))?;
```

## Reasoning

### `Reasoner`

A reasoner for OWL 2 ontologies that provides functionality for checking consistency, classifying ontologies, realizing individuals, and checking instance relationships.

#### `new`

```rust
pub fn new(ontology: Ontology) -> Self
```

Creates a new reasoner for the given ontology.

**Arguments:**
- `ontology`: The ontology to reason about.

**Returns:**
A new reasoner instance.

**Example:**
```rust
use owl2_rs::api::{load_ontology, Reasoner};

let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;

let ontology = load_ontology(ontology_str).unwrap();
let reasoner = Reasoner::new(ontology);
```

#### `is_consistent`

```rust
pub fn is_consistent(&mut self) -> bool
```

Checks if the ontology is consistent (satisfiable).

An ontology is consistent if it has at least one model, i.e., there exists an interpretation that satisfies all the axioms in the ontology.

**Returns:**
- `true`: If the ontology is consistent.
- `false`: If the ontology is inconsistent.

**Example:**
```rust
use owl2_rs::api::{load_ontology, Reasoner};

let ontology_str = r#"Ontology(<http://example.com/ontology>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;

let ontology = load_ontology(ontology_str).unwrap();
let mut reasoner = Reasoner::new(ontology);
let is_consistent = reasoner.is_consistent();
assert!(is_consistent);
```

#### `classify`

```rust
pub fn classify(&mut self) -> ClassHierarchy
```

Computes the class hierarchy for the ontology.

This method computes the subsumption relationships between classes in the ontology.

**Returns:**
The computed class hierarchy.

**Example:**
```rust
use owl2_rs::api::{load_ontology, Reasoner};

let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;

let ontology = load_ontology(ontology_str).unwrap();
let mut reasoner = Reasoner::new(ontology);
let hierarchy = reasoner.classify();
```

#### `realize`

```rust
pub fn realize(&mut self) -> HashMap<Individual, IndividualTypes>
```

Finds the most specific types for all individuals in the ontology.

This method determines the most specific classes that each individual belongs to.

**Returns:**
A mapping from individuals to their most specific types.

**Example:**
```rust
use owl2_rs::api::{load_ontology, Reasoner};
use std::collections::HashMap;

let ontology_str = r#"Ontology(<http://example.com/ontology>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;

let ontology = load_ontology(ontology_str).unwrap();
let mut reasoner = Reasoner::new(ontology);
let individual_types = reasoner.realize();
```

## Core Data Structures

### `Ontology`

Represents a complete OWL 2 ontology.

**Fields:**
- `direct_imports`: IRIs of ontologies that are directly imported by this ontology.
- `axioms`: The axioms that make up this ontology.

### `Class`

A class in an OWL 2 ontology, identified by an IRI.

### `Individual`

Represents an individual in the ontology, which can be either named (identified by an IRI) or anonymous (identified by a NodeID).

### `ClassHierarchy`

Represents the class hierarchy computed by the reasoner.

**Fields:**
- `subclasses`: Maps each class to its direct subclasses.
- `superclasses`: Maps each class to its direct superclasses.

### `IndividualTypes`

Represents the types of an individual.

**Fields:**
- `most_specific`: The most specific classes that the individual belongs to.
- `all`: All classes that the individual belongs to (including superclasses).