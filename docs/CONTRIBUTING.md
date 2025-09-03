# Contribution Guidelines for owl2_rs

This document provides guidelines for contributing to the owl2_rs library.

## Table of Contents

1. [Getting Started](#getting-started)
2. [How to Contribute](#how-to-contribute)
3. [Coding Standards](#coding-standards)
4. [Documentation](#documentation)
5. [Testing](#testing)
6. [Pull Request Process](#pull-request-process)
7. [Community](#community)

## Getting Started

To get started with contributing to owl2_rs:

1. Fork the repository
2. Clone your forked repository
3. Create a new branch for your feature or bug fix
4. Make your changes
5. Write tests for your changes
6. Update documentation as needed
7. Submit a pull request

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Git

### Building the Project

```bash
# Clone the repository
git clone https://github.com/your-username/owl2_rs.git

# Navigate to the project directory
cd owl2_rs

# Build the project
cargo build

# Run tests
cargo test
```

## How to Contribute

There are many ways to contribute to owl2_rs:

1. **Bug Reports**: Report bugs by creating issues on GitHub
2. **Feature Requests**: Suggest new features by creating issues on GitHub
3. **Code Contributions**: Implement new features or fix bugs
4. **Documentation**: Improve documentation
5. **Testing**: Write new tests or improve existing ones
6. **Examples**: Create new example programs

### Reporting Bugs

When reporting a bug, please include:

1. A clear and descriptive title
2. Steps to reproduce the bug
3. Expected behavior
4. Actual behavior
5. Environment information (Rust version, OS, etc.)
6. Any relevant code snippets or error messages

### Suggesting Features

When suggesting a new feature, please include:

1. A clear and descriptive title
2. A detailed description of the feature
3. Use cases for the feature
4. Any relevant examples or mockups

## Coding Standards

### Rust Style Guide

Follow the Rust style guide as enforced by rustfmt:

```bash
# Format your code
cargo fmt
```

### Naming Conventions

1. Use `snake_case` for variables, functions, and modules
2. Use `CamelCase` for types and traits
3. Use `SCREAMING_SNAKE_CASE` for constants
4. Use descriptive names that clearly indicate the purpose of the item

### Error Handling

1. Use `Result<T, E>` for functions that can fail
2. Use the `thiserror` crate for defining error types
3. Provide meaningful error messages
4. Handle errors appropriately - don't panic unless it's a truly unrecoverable situation

### Code Organization

1. Keep functions small and focused
2. Use modules to organize related functionality
3. Export public API items in `lib.rs`
4. Use `pub(crate)` for items that are only used within the crate

## Documentation

All public API items should have documentation comments using Rust's documentation format:

```rust
/// A class in an OWL 2 ontology.
///
/// Classes are used to represent sets of individuals. A class is identified by an IRI.
///
/// # Examples
///
/// ```
/// use owl2_rs::{Class, IRI};
///
/// let class = Class(IRI("http://example.com/Student".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Class(pub IRI);
```

### Documentation Content

1. Provide a clear description of what the item does
2. Include examples for complex functionality
3. Document all parameters and return values
4. Mention any important implementation details
5. Note any panics or error conditions

## Testing

All code changes should include appropriate tests:

1. **Unit Tests**: Test individual functions and methods
2. **Integration Tests**: Test how components work together
3. **Regression Tests**: Prevent previously fixed bugs from reappearing

### Writing Tests

1. Place unit tests in `#[cfg(test)]` modules within the same file
2. Place integration tests in the `tests/` directory
3. Use descriptive test function names
4. Test both positive and negative cases
5. Include comments explaining what each test is verifying

### Running Tests

Before submitting a pull request, ensure all tests pass:

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run tests with specific pattern
cargo test test_name_pattern
```

## Pull Request Process

1. **Create a Branch**: Create a new branch for your changes
2. **Make Changes**: Implement your feature or fix
3. **Write Tests**: Add tests for your changes
4. **Update Documentation**: Update or add documentation as needed
5. **Format Code**: Run `cargo fmt` to format your code
6. **Run Tests**: Ensure all tests pass with `cargo test`
7. **Commit Changes**: Use clear, descriptive commit messages
8. **Push Changes**: Push your branch to your fork
9. **Create Pull Request**: Submit a pull request to the main repository

### Pull Request Guidelines

1. **Title**: Use a clear, descriptive title
2. **Description**: Provide a detailed description of your changes
3. **Related Issues**: Reference any related issues
4. **Testing**: Describe how you tested your changes
5. **Breaking Changes**: Note any breaking changes to the API

### Code Review Process

1. All pull requests must be reviewed by at least one maintainer
2. Address all review comments
3. Make requested changes or provide justification for not making them
4. Once approved, a maintainer will merge the pull request

## Community

### Communication

1. **GitHub Issues**: For bug reports and feature requests
2. **GitHub Discussions**: For general discussion and questions
3. **Pull Requests**: For code contributions

### Code of Conduct

Please follow our Code of Conduct in all interactions:

1. Be respectful and inclusive
2. Be constructive in feedback
3. Focus on the code, not the person
4. Welcome newcomers and help them get started