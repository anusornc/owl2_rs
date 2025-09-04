//! Benchmark tests for the OWL 2 reasoner and profile checker.
//!
//! These benchmarks help identify performance bottlenecks in the implementation
//! and track improvements over time.

use criterion::{criterion_group, criterion_main, Criterion};
use owl2_rs::{
    api::{load_ontology, Reasoner},
    owl2_profile::{check_profile_compliance, OwlProfile},
};

/// Creates a moderately complex ontology for benchmarking
fn create_complex_ontology() -> String {
    r#"Ontology(<http://example.com/benchmark>
  SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
  SubClassOf(Class(<http://example.com/Employee>) Class(<http://example.com/Person>))
  SubClassOf(Class(<http://example.com/Manager>) Class(<http://example.com/Employee>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
  
  ObjectPropertyDomain(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Person>))
  ObjectPropertyRange(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Organization>))
  
  DataPropertyDomain(DataProperty(<http://example.com/hasAge>) Class(<http://example.com/Person>))
  DataPropertyRange(DataProperty(<http://example.com/hasAge>) Datatype(<http://www.w3.org/2001/XMLSchema#integer>))
  
  DataPropertyDomain(DataProperty(<http://example.com/hasName>) Class(<http://example.com/Person>))
  DataPropertyRange(DataProperty(<http://example.com/hasName>) Datatype(<http://www.w3.org/2001/XMLSchema#string>))
  
  FunctionalDataProperty(DataProperty(<http://example.com/hasAge>))
  
  ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
  ClassAssertion(Class(<http://example.com/Employee>) NamedIndividual(<http://example.com/mary>))
  ClassAssertion(Class(<http://example.com/Manager>) NamedIndividual(<http://example.com/bob>))
  
  ObjectPropertyAssertion(ObjectProperty(<http://example.com/hasParent>) NamedIndividual(<http://example.com/john>) NamedIndividual(<http://example.com/mary>))
  ObjectPropertyAssertion(ObjectProperty(<http://example.com/worksFor>) NamedIndividual(<http://example.com/mary>) NamedIndividual(<http://example.com/company>))
  
  DataPropertyAssertion(DataProperty(<http://example.com/hasAge>) NamedIndividual(<http://example.com/john>) "22"^^<http://www.w3.org/2001/XMLSchema#integer>)
  DataPropertyAssertion(DataProperty(<http://example.com/hasName>) NamedIndividual(<http://example.com/john>) "John Doe"^^<http://www.w3.org/2001/XMLSchema#string>)
  
  SubClassOf(
    Class(<http://example.com/Person>)
    ObjectIntersectionOf(
      ObjectSomeValuesFrom(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
      ObjectMaxCardinality(1 ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
    )
  )
)"#.to_string()
}

/// Benchmark for ontology parsing
fn bench_parse_ontology(c: &mut Criterion) {
    let ontology_str = create_complex_ontology();
    
    c.bench_function("parse_complex_ontology", |b| {
        b.iter(|| {
            let _ontology = load_ontology(&ontology_str).expect("Failed to parse ontology");
        })
    });
}

/// Benchmark for consistency checking
fn bench_consistency_check(c: &mut Criterion) {
    let ontology_str = create_complex_ontology();
    let ontology = load_ontology(&ontology_str).expect("Failed to parse ontology");
    
    c.bench_function("consistency_check", |b| {
        b.iter(|| {
            let mut reasoner = Reasoner::new(ontology.clone());
            let _is_consistent = reasoner.is_consistent();
        })
    });
}

/// Benchmark for RL profile checking
fn bench_rl_profile_check(c: &mut Criterion) {
    let ontology_str = create_complex_ontology();
    let ontology = load_ontology(&ontology_str).expect("Failed to parse ontology");
    
    c.bench_function("rl_profile_check", |b| {
        b.iter(|| {
            let _result = check_profile_compliance(&ontology, OwlProfile::RL);
        })
    });
}

/// Benchmark for EL profile checking
fn bench_el_profile_check(c: &mut Criterion) {
    let ontology_str = create_complex_ontology();
    let ontology = load_ontology(&ontology_str).expect("Failed to parse ontology");
    
    c.bench_function("el_profile_check", |b| {
        b.iter(|| {
            let _result = check_profile_compliance(&ontology, OwlProfile::EL);
        })
    });
}

/// Benchmark for complex class expression processing
fn bench_class_expression_processing(c: &mut Criterion) {
    let ontology_str = create_complex_ontology();
    let ontology = load_ontology(&ontology_str).expect("Failed to parse ontology");
    
    c.bench_function("class_expression_processing", |b| {
        b.iter(|| {
            // Process all class expressions in the ontology by doing a simple operation
            let _axioms_count = ontology.axioms.len();
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = bench_parse_ontology, bench_consistency_check, bench_rl_profile_check, bench_el_profile_check, bench_class_expression_processing
}

criterion_main!(benches);