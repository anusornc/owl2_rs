# owl2_rs

A Rust library for the Web Ontology Language (OWL 2).

## Overview

OWL2_rs is a Rust implementation of an OWL 2 parser and reasoner. It provides functionality to:

- Parse OWL 2 ontologies in Functional-Style Syntax
- Check ontology consistency
- Compute class hierarchies (classification)
- Check OWL 2 profile compliance (EL, QL, RL)
- Process complex OWL 2 constructs

## Features

- Complete OWL 2 data structure implementation
- Parser for OWL 2 Functional-Style Syntax
- Tableau-based reasoner
- Consistency checking
- Classification (subsumption hierarchy computation)
- OWL 2 profile compliance checking (EL, QL, RL)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
owl2_rs = { path = "path/to/owl2_rs" }
```

## Usage

```rust
use owl2_rs::{parser::OWLParser, reasoner::TableauReasoner};

// Parse an ontology
let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;

let ontology = OWLParser::parse_ontology(ontology_str)?;

// Check consistency
let mut reasoner = TableauReasoner::new(ontology);
let is_consistent = reasoner.is_consistent();

// Compute class hierarchy
let hierarchy = reasoner.classify();
```

## Examples

See the `examples/` directory for more detailed examples:

- `basic_reasoning.rs` - Basic reasoning functionality
- `gs1_epcis_integration.rs` - Integration with GS1 and EPCIS standards
- `uht_milk_traceability.rs` - Real-world UHT milk supply chain traceability example

## Testing

The project includes comprehensive tests:

- Unit tests for all data structures and parser components
- Integration tests with standard OWL 2 test cases
- Specialized tests for GS1 and EPCIS ontologies
- Real-world supply chain traceability tests

To run all tests:
```bash
cargo test
```

To run specific test suites:
```bash
# Run GS1/EPCIS tests
cargo test --test gs1_epcis_tests

# Run UHT milk supply chain tests
cargo test --test uht_supplychain_tests

# Run integration tests
cargo test --test integration_tests
```

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built using the [pest](https://pest.rs/) parser library