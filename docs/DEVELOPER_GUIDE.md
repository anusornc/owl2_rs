# Developer Guide for owl2_rs

This guide provides comprehensive documentation for professional developers who want to use or contribute to the owl2_rs library. The owl2_rs library is a Rust implementation of an OWL 2 parser and reasoner that provides functionality for working with OWL 2 ontologies.

## Table of Contents

1. [Overview](#overview)
2. [Installation](#installation)
3. [Basic Usage](#basic-usage)
4. [API Reference](#api-reference)
5. [Architecture](#architecture)
6. [Tutorials](#tutorials)
7. [Testing](#testing)
8. [Contributing](#contributing)
9. [Performance Considerations](#performance-considerations)

## Overview

The owl2_rs library provides functionality for working with OWL 2 ontologies, including:

- Parsing OWL 2 ontologies in Functional-Style Syntax
- Checking ontology consistency
- Computing class hierarchies (classification)
- Realizing individuals (finding their most specific types)
- Instance checking

The library is built using the pest parser library for parsing and implements a tableau-based reasoner for reasoning tasks.

## Installation

To use owl2_rs in your Rust project, add it to your `Cargo.toml`:

```toml
[dependencies]
owl2_rs = { path = "../path/to/owl2_rs" }
```

For development, clone the repository and run:

```bash
cargo build
```

## Basic Usage

Here's a simple example of how to use the library:

```rust
use owl2_rs::api::{load_ontology, Reasoner};

// Load an ontology from a string
let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
)"#;

let ontology = load_ontology(ontology_str).unwrap();

// Create a reasoner and check consistency
let mut reasoner = Reasoner::new(ontology);
let is_consistent = reasoner.is_consistent();
```

## API Reference

For detailed API documentation, see the [API Reference](API_REFERENCE.md).

## Architecture

For information about the library's architecture and design decisions, see the [Architecture Documentation](ARCHITECTURE.md).

## Tutorials

For step-by-step tutorials on common use cases, see the [Tutorials](TUTORIALS.md).

## Testing

For information about the testing framework and how to write tests, see the [Testing Documentation](TESTING.md).

## Contributing

For guidelines on how to contribute to the project, see the [Contribution Guidelines](CONTRIBUTING.md).

## Performance Considerations

For information about performance considerations and optimization tips, see the [Performance Guide](PERFORMANCE.md).