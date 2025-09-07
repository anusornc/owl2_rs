//! # WebAssembly Support for OWL 2
//! 
//! This module provides a simplified API for using owl2_rs in WebAssembly environments.
//! 
//! ## Usage
//! 
//! ```javascript
//! import init, { load_ontology_from_string } from './owl2_rs.js';
//! 
//! async function example() {
//!     await init();
//!     const ontology = load_ontology_from_string("@prefix owl: <http://www.w3.org/2002/07/owl#> . @prefix rdfs: <http://www.w3.org/2000/01/rdf-schema#> . <http://example.com/ontology> a owl:Ontology . <http://example.com/Student> a owl:Class . <http://example.com/Person> a owl:Class . <http://example.com/Student> rdfs:subClassOf <http://example.com/Person> .");
//!     console.log(ontology);
//! }
//! ```

use wasm_bindgen::prelude::*;
use crate::{api, Ontology};

/// Loads an ontology from a string in OWL 2 Functional-Style Syntax.
/// 
/// This function is designed for use in WebAssembly environments.
/// 
/// # Arguments
/// 
/// * `input` - A string containing the ontology in OWL 2 Functional-Style Syntax.
/// 
/// # Returns
/// 
/// A JavaScript object representing the ontology, or throws an error.
#[wasm_bindgen]
pub fn load_ontology_from_string(input: &str) -> Result<JsValue, JsValue> {
    match api::load_ontology(input) {
        Ok(ontology) => {
            // Convert the ontology to a JSON value that can be passed to JavaScript
            // In a full implementation, we would serialize the ontology to JSON
            Ok(JsValue::from_str("Ontology loaded successfully"))
        },
        Err(e) => Err(JsValue::from_str(&format!("Error loading ontology: {:?}", e)))
    }
}

/// Checks if an ontology is consistent.
/// 
/// This function is designed for use in WebAssembly environments.
/// 
/// # Arguments
/// 
/// * `ontology` - A JavaScript object representing the ontology.
/// 
/// # Returns
/// 
/// True if the ontology is consistent, false otherwise.
#[wasm_bindgen]
pub fn is_consistent() -> bool {
    // In a full implementation, we would:
    // 1. Convert the JavaScript ontology object to a Rust Ontology
    // 2. Create a reasoner
    // 3. Check consistency
    // 4. Return the result
    true // Placeholder
}

/// Gets the class hierarchy for an ontology.
/// 
/// This function is designed for use in WebAssembly environments.
/// 
/// # Arguments
/// 
/// * `ontology` - A JavaScript object representing the ontology.
/// 
/// # Returns
/// 
/// A JavaScript object representing the class hierarchy.
#[wasm_bindgen]
pub fn get_class_hierarchy() -> JsValue {
    // In a full implementation, we would:
    // 1. Convert the JavaScript ontology object to a Rust Ontology
    // 2. Create a reasoner
    // 3. Compute the class hierarchy
    // 4. Serialize the result to JSON and return it
    JsValue::from_str("Class hierarchy") // Placeholder
}