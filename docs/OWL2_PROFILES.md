# OWL 2 Profile Support

The owl2_rs library includes support for checking OWL 2 profile compliance. OWL 2 defines several profiles that restrict the expressivity of the language to achieve better computational properties:

## Supported Profiles

1. **OWL 2 EL** - Designed for ontologies with large numbers of individuals and classes
2. **OWL 2 QL** - Designed for querying large databases
3. **OWL 2 RL** - Designed for rule-based reasoning

## Profile Checking API

The library provides functionality to check if an ontology conforms to specific OWL 2 profiles:

```rust
use owl2_rs::owl2_profile::{check_profile_compliance, OwlProfile};

// Load an ontology
let ontology = load_ontology(ontology_str)?;

// Check if it conforms to the EL profile
let result = check_profile_compliance(&ontology, OwlProfile::EL);

if result.conforms {
    println!("Ontology conforms to OWL 2 EL profile");
} else {
    println!("Ontology does not conform to OWL 2 EL profile");
    for violation in result.violations {
        println!("Violation: {}", violation);
    }
}
```

## Profile Restrictions

### OWL 2 EL Profile

The EL profile allows:
- Class assertions
- Object property assertions
- Data property assertions
- Subclass axioms with limited class expressions (intersections and existential restrictions)
- Object property domain and range axioms
- Data property domain and range axioms
- Functional data property axioms

The EL profile does NOT allow:
- Union of classes (ObjectUnionOf)
- Complement of classes (ObjectComplementOf)
- Universal restrictions (ObjectAllValuesFrom)
- Cardinality restrictions
- HasValue restrictions with complex expressions

### OWL 2 RL Profile

The RL profile is more expressive than EL and allows:
- All constructs allowed in EL
- ObjectHasSelf construct
- Some forms of cardinality restrictions
- More object property axioms

### OWL 2 QL Profile

The QL profile is designed for query answering and has different restrictions.

## Example Usage

```rust
use owl2_rs::owl2_profile::{check_profile_compliance, OwlProfile};

// EL-compliant ontology
let el_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
)"#;

let ontology = load_ontology(el_ontology_str)?;
let result = check_profile_compliance(&ontology, OwlProfile::EL);

assert!(result.conforms);

// Non-EL compliant ontology (contains union)
let full_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
)"#;

let ontology = load_ontology(full_ontology_str)?;
let result = check_profile_compliance(&ontology, OwlProfile::EL);

assert!(!result.conforms);
```

## Testing Profile Compliance

The library includes comprehensive tests for profile checking functionality. See the `tests/` directory for examples of how to test profile compliance.