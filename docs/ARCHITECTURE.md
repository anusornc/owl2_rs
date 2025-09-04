# Architecture Documentation for owl2_rs

This document describes the architecture and design decisions of the owl2_rs library.

## Table of Contents

1. [Overview](#overview)
2. [Core Components](#core-components)
3. [Parser Design](#parser-design)
4. [Reasoner Design](#reasoner-design)
5. [Profile Checking Design](#profile-checking-design)
6. [Data Structures](#data-structures)

## Overview

The owl2_rs library is organized into three main components:

1. **Parser**: Responsible for parsing OWL 2 Functional-Style Syntax into internal data structures
2. **Data Structures**: Core Rust structs and enums that represent OWL 2 constructs
3. **Reasoner**: Implements reasoning algorithms for consistency checking, classification, and realization
4. **Profile Checker**: Implements OWL 2 profile compliance checking

## Core Components

### Parser

The parser is implemented using the `pest` crate, which provides a PEG (Parsing Expression Grammar) implementation for Rust. The grammar is defined in `src/grammar.pest` and follows the OWL 2 Functional-Style Syntax specification.

Key design decisions:
- Uses PEG for precise syntax definition
- Implements a recursive descent parser
- Separates grammar definition from parsing logic
- Provides comprehensive error handling

### Data Structures

The data structures module defines all OWL 2 constructs as Rust structs and enums. These include:

- Entities (Class, Datatype, ObjectProperty, DataProperty, etc.)
- Class expressions (ObjectIntersectionOf, ObjectUnionOf, etc.)
- Object property expressions
- Data ranges
- Axioms (ClassAxiom, ObjectPropertyAxiom, etc.)
- Assertions
- Ontology

Key design decisions:
- Uses Rust's type system for strong typing
- Implements Clone, Debug, PartialEq, Eq, and Hash traits for all structures
- Uses Box for recursive data structures to avoid infinite size
- Follows OWL 2 Structural Specification naming conventions

### Reasoner

The reasoner implements a tableau-based algorithm for reasoning about OWL 2 ontologies. It includes:

- Completion graph data structures
- Tableau expansion rules
- Consistency checking with clash detection
- Classification engine
- Realization engine

Key design decisions:
- Implements the standard tableau algorithm for ALC description logic
- Uses a graph-based representation for the completion graph
- Separates the tableau algorithm from the OWL 2 specific logic
- Provides incremental reasoning capabilities

### Profile Checker

The profile checker implements functionality for checking OWL 2 profile compliance. It includes:

- Profile definitions (EL, QL, RL, Full)
- Profile compliance checking algorithms
- Violation reporting mechanisms

Key design decisions:
- Extensible design for adding new profiles
- Detailed violation reporting
- Separation of profile checking logic from core reasoning

## Parser Design

### Grammar

The grammar is defined in `src/grammar.pest` using PEG syntax. It follows the OWL 2 Functional-Style Syntax specification with some simplifications for implementation purposes.

Key features:
- Comprehensive coverage of OWL 2 constructs
- Support for comments
- Proper handling of IRIs and literals
- Recursive definitions for complex expressions

### Parser Implementation

The parser implementation in `src/parser.rs` follows these principles:

1. **Modular Design**: Each OWL 2 construct has its own parsing function
2. **Error Handling**: Uses pest's error handling mechanisms
3. **Type Safety**: Converts parsed text directly into strongly-typed data structures
4. **Extensibility**: Easy to add new parsing functions for additional constructs

## Reasoner Design

### Tableau Algorithm

The reasoner implements a tableau-based algorithm for checking consistency and computing inferences. The main components are:

1. **Completion Graph**: A graph-based representation of individuals and their concepts/roles
2. **Expansion Rules**: Rules that add new concepts or role assertions to the graph
3. **Clash Detection**: Mechanism to detect inconsistencies in the graph
4. **Subsumption Checking**: Uses the standard technique of checking if C ⊓ ¬D is unsatisfiable

### Data Structures

#### `TableauReasoner`

The main reasoner struct that contains:
- The ontology to reason about
- The completion graph
- Reasoning state

#### `CompletionGraph`

Manages:
- Nodes representing individuals
- Concepts associated with nodes
- Role relationships between nodes
- Fresh individual generation

#### `Node`

Represents an individual with:
- The individual identifier
- Concepts (class expressions) the individual is an instance of
- Role relationships to other individuals

### Implemented Rules

1. **Conjunction Rule**: If an individual is an instance of A ⊓ B, then it is also an instance of both A and B
2. **Disjunction Rule**: If an individual is an instance of A ⊔ B, then it is also an instance of either A or B
3. **Existential Rule**: If an individual is an instance of ∃R.C, then there must exist another individual connected via R that is an instance of C
4. **Universal Rule**: If an individual is an instance of ∀R.C, then for every individual connected via R, that individual must be an instance of C

## Profile Checking Design

### OWL 2 Profiles

The profile checker implements checking for the main OWL 2 profiles:

1. **OWL 2 EL**: Designed for ontologies with large numbers of individuals
2. **OWL 2 QL**: Designed for querying large databases
3. **OWL 2 RL**: Designed for rule-based reasoning

### Design Principles

1. **Extensibility**: Easy to add new profiles or extend existing ones
2. **Detailed Reporting**: Provides specific violation information
3. **Performance**: Efficient checking algorithms
4. **Accuracy**: Correct implementation of profile restrictions

### Implementation

The profile checking is implemented in `src/owl2_profile.rs` and includes:

- Profile definitions as Rust enums
- Compliance checking functions for each profile
- Violation reporting mechanisms
- Helper functions for checking specific constructs

## Data Structures

### IRI

An Internationalized Resource Identifier (IRI) used throughout OWL 2 to identify entities.

### ClassExpression

Represents a class or a boolean combination of classes with variants:
- `Class(Class)`
- `ObjectIntersectionOf(Vec<ClassExpression>)`
- `ObjectUnionOf(Vec<ClassExpression>)`
- `ObjectComplementOf(Box<ClassExpression>)`
- `ObjectOneOf(Vec<Individual>)`
- `ObjectSomeValuesFrom { property: ObjectPropertyExpression, filler: Box<ClassExpression> }`
- `ObjectAllValuesFrom { property: ObjectPropertyExpression, filler: Box<ClassExpression> }`
- `ObjectHasValue { property: ObjectPropertyExpression, value: Individual }`
- `ObjectHasSelf(ObjectPropertyExpression)`
- `ObjectMinCardinality { min: u32, property: ObjectPropertyExpression, filler: Option<Box<ClassExpression>> }`
- `ObjectMaxCardinality { max: u32, property: ObjectPropertyExpression, filler: Option<Box<ClassExpression>> }`
- `ObjectExactCardinality { cardinality: u32, property: ObjectPropertyExpression, filler: Option<Box<ClassExpression>> }`

### ObjectPropertyExpression

Represents an object property or an inverse of an object property with variants:
- `ObjectProperty(ObjectProperty)`
- `InverseObjectProperty(ObjectProperty)`
- `ObjectPropertyChain(Vec<ObjectPropertyExpression>)`

### Axioms

Axioms are the statements that make up an ontology:
- `ClassAxiom`: Axioms about classes
- `ObjectPropertyAxiom`: Axioms about object properties
- `DataPropertyAxiom`: Axioms about data properties
- `Assertion`: Assertions about individuals