# Project: owl2_rs - A Rust Library for OWL2

This document outlines the main tasks for creating a comprehensive Rust library for the Web Ontology Language (OWL2), including a reasoner.

## Task 1: Core Data Structures
Define and implement the fundamental Rust structs and enums to represent every component of an OWL2 ontology. This includes entities, axioms, and other constructs as defined in the OWL2 specification.

## Task 2: OWL2 Parser
Implement a parser for one of the standard OWL2 syntaxes. We will start with the Functional-Style Syntax as it is the most direct mapping to the OWL2 structural specification.

- **Sub-task 2.1:** Investigate and select a suitable parsing library (e.g., `nom`, `pest`).
- **Sub-task 2.2:** Implement the parser for OWL2 Functional-Style Syntax.
- **Sub-task 2.3 (Future):** Plan for parsers for other syntaxes like RDF/XML and Manchester.

## Task 3: Basic Reasoner Implementation
Develop a basic reasoning engine based on the tableau algorithm, similar to HermiT. The initial focus will be on core reasoning tasks.

- **Sub-task 3.1:** Implement the core data structures for the tableau reasoner.
- **Sub-task 3.2:** Implement the tableau expansion rules.
- **Sub-task 3.3:** Implement a consistency checker (to determine if an ontology has any contradictions).

## Task 4: Advanced Reasoner Features
Extend the reasoner to support more advanced tasks.

- **Sub-task 4.1:** Implement a classification engine to compute the class hierarchy.
- **Sub-task 4.2:** Implement a realization engine to find the types of individuals.
- **Sub-task 4.3:** Implement instance checking. (COMPLETED)

## Task 5: Testing Framework
A comprehensive testing suite is crucial for ensuring correctness.

- **Sub-task 5.1:** Develop unit tests for all data structures and axioms.
- **Sub-task 5.2:** Create integration tests for the parser using standard OWL2 test cases.
- **Sub-task 5.3:** Develop tests for the reasoner, verifying its correctness against known results.

## Task 6: Documentation and API Design
Create clear, user-friendly documentation and a well-designed public API.

- **Sub-task 6.1:** Add Rustdoc comments to all public items.
- **Sub-task 6.2:** Create a user guide with examples of how to use the library.
- **Sub-task 6.3:** Refine the public API for ease of use.
