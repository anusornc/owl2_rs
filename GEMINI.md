# GEMINI.md: Project Context for OWL2_rs

This file provides context for the `OWL2_rs` project, a Rust library for working with the Web Ontology Language (OWL2).

## Project Overview

`OWL2_rs` is a new Rust library aiming to provide a complete implementation of the OWL2 W3C standard. The project is in its initial phase and is being built from the ground up.

### Current Status (2025-09-02)

- **Task 1 (Complete):** The core data structures for all OWL2 entities and axioms have been implemented in `src/lib.rs`.
- **Task 2 (In Progress):** The initial setup for the `pest` parser has been completed. The `src/parser.rs` and `src/grammar.pest` files have been created. IRI, Prefix, Entity, Literal, Class Expression, Property Expression, and all Axiom types parsing is implemented and tested.
- The corresponding items in `Plan.md` have been checked off.

The key goals, as outlined in `Project.md` and `Plan.md`, are:
1.  **Core Data Structures:** Create a robust set of Rust `structs` and `enums` that map directly to OWL2 constructs (Axioms, Entities, etc.).
2.  **Parser:** Implement a parser for OWL2 syntaxes, starting with the Functional-Style Syntax.
3.  **Reasoner:** Build a powerful tableau-based reasoner, similar in functionality to HermiT, capable of tasks like consistency checking, classification, and realization.
4.  **Testing:** Ensure correctness through a comprehensive testing framework, including unit, integration, and conformance tests.

The project is structured around a detailed development plan in `Plan.md`, which serves as a checklist for implementation.

## Building and Running

This is a standard Rust library project managed by Cargo.

*   **Check for errors:**
    ```bash
    cargo check
    ```
*   **Build the library:**
    ```bash
    cargo build
    ```
*   **Run tests:**
    ```bash
    cargo test
    ```
*   **Build documentation:**
    ```bash
    cargo doc --open
    ```

## Development Conventions

*   **Planning:** All development should follow the detailed tasks outlined in `Plan.md`.
*   **Coding Style:** Adhere to standard Rust formatting and conventions. Run `cargo fmt` to format the code.
*   **Linting:** Use `cargo clippy` to catch common mistakes and improve the code.
*   **Testing:** Every new feature (e.g., a data structure, a parser component, a reasoner rule) should be accompanied by corresponding unit tests.
*   **API:** The public API should be carefully designed for clarity and ease of use, as specified in `Plan.md`. All public items must be documented with Rustdoc comments.
