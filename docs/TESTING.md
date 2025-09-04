# Testing Documentation for owl2_rs

This document describes the testing framework for the owl2_rs library and how to write tests.

## Table of Contents

1. [Overview](#overview)
2. [Running Tests](#running-tests)
3. [Test Structure](#test-structure)
4. [Writing Unit Tests](#writing-unit-tests)
5. [Writing Integration Tests](#writing-integration-tests)
6. [Profile Testing](#profile-testing)
7. [Test Cases](#test-cases)

## Overview

The owl2_rs library has a comprehensive test suite that includes:

- Unit tests for all data structures and parser components
- Integration tests with standard OWL 2 test cases
- Tests for the reasoner with known consistency/inconsistency results
- Tests for classification with known class hierarchies
- Tests for OWL 2 profile compliance checking

The tests are organized into several categories:
- Unit tests in each module (e.g., `src/lib.rs`, `src/parser.rs`, `src/reasoner.rs`)
- Integration tests in the `tests/` directory
- Example programs in the `examples/` directory

## Running Tests

To run all tests:

```bash
cargo test
```

To run specific test suites:

```bash
# Run unit tests only
cargo test --lib

# Run integration tests only
cargo test --test integration_tests

# Run profile tests only
cargo test --test owl2_profile_tests
cargo test --test comprehensive_profile_tests

# Run tests with a specific name pattern
cargo test test_basic_consistency

# Run tests in a specific file
cargo test --test integration_tests test_gs1_ontology_parsing
```

To run examples:

```bash
# Run a specific example
cargo run --example basic_reasoning
```

## Test Structure

### Unit Tests

Unit tests are located within each module file using the `#[cfg(test)]` attribute. For example, in `src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iri_creation() {
        let iri = IRI("http://example.com/class".to_string());
        assert_eq!(iri.0, "http://example.com/class");
    }
}
```

### Integration Tests

Integration tests are located in the `tests/` directory. These tests typically:

1. Load ontology files
2. Parse the ontologies
3. Run reasoning tasks
4. Verify the results

For example, in `tests/integration_tests.rs`:

```rust
#[test]
fn test_basic_consistency() {
    let test_case = OWL2TestCase {
        name: "Basic Consistency Test".to_string(),
        ontology_str: r#"Ontology(<http://example.com/test>
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#.to_string(),
        expected_consistent: true,
    };
    
    run_owl2_test_case(test_case);
}
```

## Writing Unit Tests

When writing unit tests for new functionality, follow these guidelines:

1. Place tests in a `#[cfg(test)]` module within the same file
2. Use descriptive test function names that start with `test_`
3. Test both positive and negative cases
4. Test edge cases and error conditions
5. Use `assert_eq!`, `assert_ne!`, `assert!` and other assertion macros appropriately

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_creation() {
        let iri = IRI("http://example.com/MyClass".to_string());
        let class = Class(iri);
        assert_eq!(class.0 .0, "http://example.com/MyClass");
    }

    #[test]
    #[should_panic(expected = "Expected error message")]
    fn test_error_condition() {
        // Test code that should panic
    }
}
```

## Writing Integration Tests

When writing integration tests, follow these guidelines:

1. Create test cases that represent real-world scenarios
2. Use the `OWL2TestCase` struct for consistency
3. Test both consistent and inconsistent ontologies
4. Verify reasoning results against expected outcomes
5. Include comments explaining what each test is verifying

Example:

```rust
#[test]
fn test_subclass_relationship() {
    let test_case = OWL2TestCase {
        name: "Subclass Relationship Test".to_string(),
        ontology_str: r#"Ontology(<http://example.com/test>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#.to_string(),
        expected_consistent: true,
    };
    
    run_owl2_test_case(test_case);
}
```

## Profile Testing

The library includes comprehensive tests for OWL 2 profile compliance checking:

1. **Profile Compliance Tests**: Test that ontologies correctly identified as compliant/non-compliant
2. **Violation Reporting Tests**: Test that specific violations are correctly reported
3. **Edge Case Tests**: Test boundary conditions and complex ontology structures

Profile tests are located in:
- `tests/owl2_profile_tests.rs`: Basic profile checking tests
- `tests/comprehensive_profile_tests.rs`: Comprehensive profile checking tests

Example profile test:

```rust
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
```

## Test Cases

### OWL 2 Conformance Tests

The library includes tests based on standard OWL 2 conformance test cases. These tests verify that the parser and reasoner correctly handle OWL 2 constructs.

Test files are located in the `test_cases/` directory:
- `gs1_test.ofn`: Tests based on GS1 ontologies
- `epcis_test.ofn`: Tests based on EPCIS ontologies
- `uht_milk_supplychain.ofn`: Tests based on a real-world supply chain ontology

### Reasoner Tests

The reasoner tests verify that reasoning tasks produce correct results:

1. **Consistency Tests**: Verify that consistent ontologies are identified as consistent and inconsistent ontologies as inconsistent
2. **Classification Tests**: Verify that class hierarchies are computed correctly
3. **Realization Tests**: Verify that individual types are determined correctly
4. **Instance Checking Tests**: Verify that instance relationships are determined correctly

### Performance Tests

Performance tests measure the efficiency of parsing and reasoning operations. These tests help identify performance bottlenecks and verify that optimizations are effective.

Example performance test pattern:

```rust
#[test]
fn test_parsing_performance() {
    let ontology_str = std::fs::read_to_string("test_cases/large_ontology.ofn").unwrap();
    
    let start = std::time::Instant::now();
    let ontology = OWLParser::parse_ontology(&ontology_str).unwrap();
    let duration = start.elapsed();
    
    assert!(duration.as_secs() < 10); // Should parse in less than 10 seconds
    println!("Parsed ontology with {} axioms in {:?}", ontology.axioms.len(), duration);
}
```