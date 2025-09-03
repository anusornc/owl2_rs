# owl2_rs Documentation

This directory contains the source files for the owl2_rs documentation. The documentation is organized into several parts:

## Documentation Structure

1. **User Guide** - Conceptual documentation for using the library
2. **Tutorials** - Step-by-step examples for common tasks
3. **API Reference** - Automatically generated from source code
4. **Architecture** - Information about the library's design
5. **Contributing** - Guidelines for contributing to the project

## Building Documentation

To build the documentation locally, you'll need:

1. Rust and Cargo (latest stable version)
2. mdBook (for user guide)

### Building API Documentation

```bash
cargo doc --open
```

This will generate the API documentation and open it in your browser.

### Building User Guide

```bash
# Install mdBook if you haven't already
cargo install mdbook

# Build the documentation
mdbook build

# Serve the documentation locally
mdbook serve
```

## Documentation Tools

This project uses two main documentation tools:

1. **Cargo Doc** - For API documentation automatically generated from Rust source code
2. **mdBook** - For user guides, tutorials, and conceptual documentation

## Contributing to Documentation

To contribute to the documentation:

1. For API documentation, update the Rust source code comments
2. For user guides, edit the Markdown files in this directory
3. Follow the style guide in CONTRIBUTING.md
4. Submit a pull request with your changes

## Directory Structure

- `src/` - Source files for the documentation
- `book.toml` - Configuration file for mdBook
- `README.md` - This fileThis file was last updated on September 3, 2025
