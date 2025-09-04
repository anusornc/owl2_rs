# Tutorials for owl2_rs

This document provides step-by-step tutorials for common use cases of the owl2_rs library.

## Table of Contents

1. [Parsing a Simple Ontology](#parsing-a-simple-ontology)
2. [Checking Ontology Consistency](#checking-ontology-consistency)
3. [Computing Class Hierarchies](#computing-class-hierarchies)
4. [Realizing Individuals](#realizing-individuals)
5. [Instance Checking](#instance-checking)
6. [Working with Complex Ontologies](#working-with-complex-ontologies)
7. [Checking OWL 2 Profile Compliance](#checking-owl-2-profile-compliance)

## Parsing a Simple Ontology

This tutorial shows how to parse a simple OWL 2 ontology from a string.

```rust
use owl2_rs::api::load_ontology;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a simple ontology string
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      SubClassOf(Class(<http://example.com/Employee>) Class(<http://example.com/Person>))
      
      DisjointClasses(Class(<http://example.com/Student>) Class(<http://example.com/Employee>))
      
      ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(ontology_str)?;
    
    // Print information about the ontology
    println!("Parsed ontology with {} axioms", ontology.axioms.len());
    
    Ok(())
}
```

## Checking Ontology Consistency

This tutorial shows how to check if an ontology is consistent (satisfiable).

```rust
use owl2_rs::api::{load_ontology, Reasoner};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define an ontology string
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(ontology_str)?;
    
    // Create a reasoner
    let mut reasoner = Reasoner::new(ontology);
    
    // Check consistency
    let is_consistent = reasoner.is_consistent();
    
    if is_consistent {
        println!("The ontology is consistent");
    } else {
        println!("The ontology is inconsistent");
    }
    
    Ok(())
}
```

## Computing Class Hierarchies

This tutorial shows how to compute the class hierarchy (classification) of an ontology.

```rust
use owl2_rs::api::{load_ontology, Reasoner};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define an ontology string
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      SubClassOf(Class(<http://example.com/Employee>) Class(<http://example.com/Person>))
      SubClassOf(Class(<http://example.com/GraduateStudent>) Class(<http://example.com/Student>))
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(ontology_str)?;
    
    // Create a reasoner
    let mut reasoner = Reasoner::new(ontology);
    
    // Compute class hierarchy
    let hierarchy = reasoner.classify();
    
    // Print information about the hierarchy
    println!("Found {} classes with superclasses", hierarchy.superclasses.len());
    println!("Found {} classes with subclasses", hierarchy.subclasses.len());
    
    // Print superclasses for each class
    for (class, superclasses) in &hierarchy.superclasses {
        println!("Class {:?} has superclasses: {:?}", class, superclasses);
    }
    
    // Print subclasses for each class
    for (class, subclasses) in &hierarchy.subclasses {
        println!("Class {:?} has subclasses: {:?}", class, subclasses);
    }
    
    Ok(())
}
```

## Realizing Individuals

This tutorial shows how to find the most specific types for all individuals in an ontology.

```rust
use owl2_rs::api::{load_ontology, Reasoner};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define an ontology string
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      SubClassOf(Class(<http://example.com/GraduateStudent>) Class(<http://example.com/Student>))
      
      ClassAssertion(Class(<http://example.com/GraduateStudent>) NamedIndividual(<http://example.com/john>))
      ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/jane>))
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(ontology_str)?;
    
    // Create a reasoner
    let mut reasoner = Reasoner::new(ontology);
    
    // Realize individuals
    let individual_types = reasoner.realize();
    
    // Print information about the individuals
    println!("Found types for {} individuals", individual_types.len());
    
    for (individual, types) in individual_types.iter() {
        println!("Individual: {:?}", individual);
        println!("  Most specific types: {:?}", types.most_specific);
        println!("  All types: {:?}", types.all);
    }
    
    Ok(())
}
```

## Instance Checking

This tutorial shows how to check if an individual is an instance of a specific class.

```rust
use owl2_rs::api::{load_ontology, Reasoner};
use owl2_rs::{Individual, IRI, Class};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define an ontology string
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(ontology_str)?;
    
    // Create a reasoner
    let mut reasoner = Reasoner::new(ontology);
    
    // Define the individual and classes to check
    let individual_john = Individual::Named(IRI("http://example.com/john".to_string()));
    let class_student = Class(IRI("http://example.com/Student".to_string()));
    let class_person = Class(IRI("http://example.com/Person".to_string()));
    
    // Check instance relationships
    let is_john_student = reasoner.is_instance_of(&individual_john, &class_student);
    let is_john_person = reasoner.is_instance_of(&individual_john, &class_person);
    
    println!("Is john an instance of Student? {}", is_john_student);
    println!("Is john an instance of Person? {}", is_john_person);
    
    Ok(())
}
```

## Working with Complex Ontologies

This tutorial shows how to work with more complex ontologies that include object properties and data properties.

```rust
use owl2_rs::api::{load_ontology, Reasoner};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define a complex ontology string
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      SubClassOf(Class(<http://example.com/Employee>) Class(<http://example.com/Person>))
      
      DisjointClasses(Class(<http://example.com/Student>) Class(<http://example.com/Employee>))
      
      ObjectPropertyDomain(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Employee>))
      ObjectPropertyRange(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Organization>))
      
      DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
      DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
      
      ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
      DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(ontology_str)?;
    
    // Print information about the ontology
    println!("Parsed ontology with {} axioms", ontology.axioms.len());
    
    // Create a reasoner and check consistency
    let mut reasoner = Reasoner::new(ontology);
    let is_consistent = reasoner.is_consistent();
    
    println!("Ontology is consistent: {}", is_consistent);
    
    // Compute class hierarchy
    let hierarchy = reasoner.classify();
    println!("Computed class hierarchy with {} superclass relationships", hierarchy.superclasses.len());
    
    // Realize individuals
    let individual_types = reasoner.realize();
    println!("Realized {} individuals", individual_types.len());
    
    Ok(())
}
```

## Checking OWL 2 Profile Compliance

This tutorial shows how to check if an ontology conforms to OWL 2 profiles.

```rust
use owl2_rs::api::load_ontology;
use owl2_rs::owl2_profile::{check_profile_compliance, OwlProfile};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Define an EL-compliant ontology
    let el_ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
      
      ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
      ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
      
      DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
      DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
      
      FunctionalDataProperty(DataProperty(<http://example.com/hasAge>))
      
      ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
      ObjectPropertyAssertion(ObjectProperty(<http://example.com/hasParent>) NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/mary>))
      DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
    )"#;
    
    // Parse the ontology
    let ontology = load_ontology(el_ontology_str)?;
    
    // Check if it conforms to the EL profile
    let el_result = check_profile_compliance(&ontology, OwlProfile::EL);
    
    if el_result.conforms {
        println!("Ontology conforms to OWL 2 EL profile");
    } else {
        println!("Ontology does not conform to OWL 2 EL profile");
        for violation in &el_result.violations {
            println!("  Violation: {}", violation);
        }
    }
    
    // Define a full OWL 2 ontology with union (not EL-compliant)
    let full_ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
    )"#;
    
    // Parse the ontology
    let full_ontology = load_ontology(full_ontology_str)?;
    
    // Check if it conforms to the EL profile
    let full_result = check_profile_compliance(&full_ontology, OwlProfile::EL);
    
    if full_result.conforms {
        println!("Full ontology conforms to OWL 2 EL profile");
    } else {
        println!("Full ontology does not conform to OWL 2 EL profile");
        for violation in &full_result.violations {
            println!("  Violation: {}", violation);
        }
    }
    
    Ok(())
}