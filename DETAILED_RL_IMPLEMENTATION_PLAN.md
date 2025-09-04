# Detailed RL Profile Implementation Plan

## Phase 1: Debugging and Fixing Parser Issues

### Task 1.1: Analyze ParseIntError in Cardinality Expressions

#### Current Error:
```
thread 'test_rl_profile_cardinality_restrictions' panicked at src/parser.rs:167:71:
called `Result::unwrap()` on an `Err` value: ParseIntError { kind: InvalidDigit }
```

#### Root Cause Analysis:
The error occurs when trying to parse a string as a u32 but getting an invalid digit. This suggests that the parser is trying to parse something that isn't a valid number.

#### Debugging Steps:

1. **Examine the grammar rule for ObjectMaxCardinality**:
   ```
   object_max_cardinality = { "ObjectMaxCardinality(" ~ (ASCII_DIGIT+)+ ~ object_property_expression ~ class_expression? ~ ")" }
   ```

2. **Check the parsing logic**:
   ```rust
   Rule::object_max_cardinality => {
       let mut inner = inner_rule_pair.into_inner();
       let max: u32 = inner.next().unwrap().as_str().parse().unwrap();
       // ...
   }
   ```

3. **Identify the issue**: 
   The parser is trying to parse the first token as a u32, but it's getting something that's not a valid number.

#### Solution Implementation:

1. **Update the grammar** to properly separate the numeric value:
   ```
   object_max_cardinality = { "ObjectMaxCardinality(" ~ ASCII_DIGIT+ ~ object_property_expression ~ class_expression? ~ ")" }
   ```

2. **Fix the parsing logic** to properly extract the numeric value:
   ```rust
   Rule::object_max_cardinality => {
       let mut inner = inner_rule_pair.into_inner();
       let max_str = inner.next().unwrap().as_str();
       let max: u32 = max_str.parse().map_err(|e| {
           Box::new(pest::error::Error::CustomErrorVariant {
               message: format!("Failed to parse cardinality '{}': {}", max_str, e),
           })
       })?;
       let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
       let filler = if let Some(filler_pair) = inner.next() {
           Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str())?))
       } else {
           None
       };
       ClassExpression::ObjectMaxCardinality { max, property, filler }
   }
   ```

### Task 1.2: Fix ObjectMinCardinality and ObjectExactCardinality Parsing

Apply the same fix to ObjectMinCardinality and ObjectExactCardinality:

```rust
Rule::object_min_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let min_str = inner.next().unwrap().as_str();
    let min: u32 = min_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", min_str, e),
        })
    })?;
    let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str())?))
    } else {
        None
    };
    ClassExpression::ObjectMinCardinality { min, property, filler }
},

Rule::object_exact_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let cardinality_str = inner.next().unwrap().as_str();
    let cardinality: u32 = cardinality_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", cardinality_str, e),
        })
    })?;
    let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str())?))
    } else {
        None
    };
    ClassExpression::ObjectExactCardinality { cardinality, property, filler }
}
```

### Task 1.3: Fix Data Property Cardinality Parsing

Apply similar fixes to data property cardinality expressions:

```rust
Rule::data_min_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let min_str = inner.next().unwrap().as_str();
    let min: u32 = min_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", min_str, e),
        })
    })?;
    let property = OWLParser::parse_data_property(inner.next().unwrap().as_str())?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(OWLParser::parse_data_range(filler_pair.as_str())?)
    } else {
        None
    };
    ClassExpression::DataMinCardinality { min, property, filler }
},

Rule::data_max_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let max_str = inner.next().unwrap().as_str();
    let max: u32 = max_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", max_str, e),
        })
    })?;
    let property = OWLParser::parse_data_property(inner.next().unwrap().as_str())?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(OWLParser::parse_data_range(filler_pair.as_str())?)
    } else {
        None
    };
    ClassExpression::DataMaxCardinality { max, property, filler }
},

Rule::data_exact_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let cardinality_str = inner.next().unwrap().as_str();
    let cardinality: u32 = cardinality_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", cardinality_str, e),
        })
    })?;
    let property = OWLParser::parse_data_property(inner.next().unwrap().as_str())?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(OWLParser::parse_data_range(filler_pair.as_str())?)
    } else {
        None
    };
    ClassExpression::DataExactCardinality { cardinality, property, filler }
}
```

## Phase 2: Fix Missing Enum Variants

### Task 2.1: Add HasKey Variant to Assertion Enum

Update `src/lib.rs` to add the missing HasKey variant:

```rust
pub enum Assertion {
    SameIndividual {
        individuals: Vec<Individual>,
    },
    DifferentIndividuals {
        individuals: Vec<Individual>,
    },
    ClassAssertion {
        class: ClassExpression,
        individual: Individual,
    },
    ObjectPropertyAssertion {
        property: ObjectPropertyExpression,
        source: Individual,
        target: Individual,
    },
    DataPropertyAssertion {
        property: DataProperty,
        source: Individual,
        target: Literal,
    },
    NegativeObjectPropertyAssertion {
        property: ObjectPropertyExpression,
        source: Individual,
        target: Individual,
    },
    NegativeDataPropertyAssertion {
        property: DataProperty,
        source: Individual,
        target: Literal,
    },
    HasKey {
        class: Class,
        object_property_expression: Vec<ObjectPropertyExpression>,
        data_property: Vec<DataProperty>,
    },
}
```

### Task 2.2: Add ObjectHasSelf Variant to ClassExpression Enum

Update `src/lib.rs` to ensure ObjectHasSelf is properly defined:

```rust
pub enum ClassExpression {
    Class(Class),
    ObjectIntersectionOf(Vec<ClassExpression>),
    ObjectUnionOf(Vec<ClassExpression>),
    ObjectComplementOf(Box<ClassExpression>),
    ObjectOneOf(Vec<Individual>),
    ObjectSomeValuesFrom {
        property: ObjectPropertyExpression,
        filler: Box<ClassExpression>,
    },
    ObjectAllValuesFrom {
        property: ObjectPropertyExpression,
        filler: Box<ClassExpression>,
    },
    ObjectHasValue {
        property: ObjectPropertyExpression,
        value: Individual,
    },
    ObjectHasSelf(ObjectPropertyExpression),
    ObjectMinCardinality {
        min: u32,
        property: ObjectPropertyExpression,
        filler: Option<Box<ClassExpression>>,
    },
    ObjectMaxCardinality {
        max: u32,
        property: ObjectPropertyExpression,
        filler: Option<Box<ClassExpression>>,
    },
    ObjectExactCardinality {
        cardinality: u32,
        property: ObjectPropertyExpression,
        filler: Option<Box<ClassExpression>>,
    },
}
```

## Phase 3: Implement RL-Specific Validation Logic

### Task 3.1: Complete RL Class Expression Validation

Update `src/owl2_profile.rs` to properly implement RL class expression validation:

```rust
/// Checks if a class expression is RL-compliant
fn is_rl_class_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            sub_exprs.iter().all(|sub_expr| is_rl_class_expression(sub_expr))
        },
        ClassExpression::ObjectUnionOf(sub_exprs) => {
            sub_exprs.iter().all(|sub_expr| is_rl_class_expression(sub_expr))
        },
        ClassExpression::ObjectComplementOf(sub_expr) => {
            is_rl_class_expression(sub_expr)
        },
        ClassExpression::ObjectOneOf(individuals) => {
            !individuals.is_empty()
        },
        ClassExpression::ObjectSomeValuesFrom { property, filler } => {
            is_rl_object_property_expression(property) && is_rl_class_expression(filler)
        },
        ClassExpression::ObjectAllValuesFrom { property, filler } => {
            is_rl_object_property_expression(property) && is_rl_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property, value: _ } => {
            is_rl_object_property_expression(property)
        },
        ClassExpression::ObjectHasSelf(property) => {
            is_rl_object_property_expression(property)
        },
        ClassExpression::ObjectMinCardinality { min, property, filler } => {
            // Only min 0 or 1 allowed in RL
            *min <= 1 && 
            is_rl_object_property_expression(property) && 
            filler.as_ref().map_or(true, |f| is_rl_class_expression(f))
        },
        ClassExpression::ObjectMaxCardinality { max, property, filler } => {
            // Only max 0 or 1 allowed in RL
            *max <= 1 && 
            is_rl_object_property_expression(property) && 
            filler.as_ref().map_or(true, |f| is_rl_class_expression(f))
        },
        ClassExpression::ObjectExactCardinality { cardinality, property, filler } => {
            // Only exact 0 or 1 allowed in RL
            *cardinality <= 1 && 
            is_rl_object_property_expression(property) && 
            filler.as_ref().map_or(true, |f| is_rl_class_expression(f))
        },
        // All other class expressions are not RL-compliant
        _ => false,
    }
}

/// Checks if an object property expression is RL-compliant
fn is_rl_object_property_expression(expr: &ObjectPropertyExpression) -> bool {
    match expr {
        ObjectPropertyExpression::ObjectProperty(_) => true,
        ObjectPropertyExpression::InverseObjectProperty(_) => true,
        // Property chains are not RL-compliant
        ObjectPropertyExpression::ObjectPropertyChain(_) => false,
    }
}
```

### Task 3.2: Update RL Axiom Validation

Update `src/owl2_profile.rs` to properly validate RL axioms:

```rust
/// Checks if an object property axiom is RL-compliant
fn check_rl_object_property_axiom(axiom: &ObjectPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        ObjectPropertyAxiom::ReflexiveObjectProperty { property: _ } => {
            // ReflexiveObjectProperty is not allowed in RL
            violations.push("ReflexiveObjectProperty axiom is not allowed in RL profile".to_string());
        },
        ObjectPropertyAxiom::IrreflexiveObjectProperty { property: _ } => {
            // IrreflexiveObjectProperty is not allowed in RL
            violations.push("IrreflexiveObjectProperty axiom is not allowed in RL profile".to_string());
        },
        // All other object property axioms are allowed in RL
        _ => {},
    }
}

/// Checks if a data property axiom is RL-compliant
fn check_rl_data_property_axiom(axiom: &DataPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        DataPropertyAxiom::FunctionalDataProperty { property: _ } => {
            // All functional data property axioms are RL-compliant
        },
        DataPropertyAxiom::DataPropertyDomain { property: _, domain } => {
            if !is_rl_class_expression(domain) {
                violations.push("DataPropertyDomain axiom has non-RL domain expression".to_string());
            }
        },
        DataPropertyAxiom::DataPropertyRange { property: _, range } => {
            if !is_rl_valid_data_range(range) {
                violations.push("DataPropertyRange axiom has non-RL range expression".to_string());
            }
        },
        // All other data property axioms are allowed in RL
        _ => {},
    }
}

/// Checks if an assertion is RL-compliant
fn check_rl_assertion(assertion: &Assertion, violations: &mut Vec<String>) {
    match assertion {
        Assertion::ClassAssertion { class, individual: _ } => {
            if !is_rl_class_expression(class) {
                violations.push("ClassAssertion has non-RL class expression".to_string());
            }
        },
        Assertion::HasKey { class, object_property_expression, data_property } => {
            // HasKey is allowed in RL with restrictions
            if !is_rl_valid_class(class) {
                violations.push("HasKey axiom has non-RL class".to_string());
            }
            for prop in object_property_expression {
                if !is_rl_object_property_expression(prop) {
                    violations.push("HasKey axiom has non-RL object property expression".to_string());
                }
            }
            for prop in data_property {
                // All data properties are RL-compliant
            }
        },
        // All other assertions are allowed in RL
        _ => {},
    }
}

/// Checks if a class is RL-valid
fn is_rl_valid_class(class: &Class) -> bool {
    let iri = &class.0.0;
    // RL does not support owl:real and owl:rational
    !iri.contains("owl:real") && !iri.contains("owl:rational")
}

/// Checks if a data range is RL-valid
fn is_rl_valid_data_range(range: &DataRange) -> bool {
    match range {
        DataRange::Datatype(datatype) => {
            let iri = &datatype.0.0;
            // RL does not support owl:real and owl:rational
            !iri.contains("owl:real") && !iri.contains("owl:rational")
        },
        DataRange::DataIntersectionOf(sub_ranges) => {
            sub_ranges.iter().all(|sub_range| is_rl_valid_data_range(sub_range))
        },
        // All other data ranges are not RL-compliant
        _ => false,
    }
}
```

## Phase 4: Testing and Validation

### Task 4.1: Fix Failing RL Tests

Update the failing RL tests to ensure they work correctly:

1. **Fix test_rl_profile_cardinality_restrictions**:
   ```rust
   #[test]
   fn test_rl_profile_cardinality_restrictions() {
       // Test RL cardinality restrictions (only max 0 or 1 allowed)
       let ontology_str = r#"Ontology(<http://example.com/ontology>
         SubClassOf(Class(<http://example.com/Student>) ObjectMaxCardinality(1 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>)))
       )"#;
       
       let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
       let result = check_profile_compliance(&ontology, OwlProfile::RL);
       
       assert!(result.conforms, "Ontology with ObjectMaxCardinality(1) should conform to RL profile. Violations: {:?}", result.violations);
       
       // Test with max 2 (should violate RL)
       let ontology_str2 = r#"Ontology(<http://example.com/ontology>
         SubClassOf(Class(<http://example.com/Student>) ObjectMaxCardinality(2 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>)))
       )"#;
       
       let ontology2 = load_ontology(ontology_str2).expect("Failed to parse ontology");
       let result2 = check_profile_compliance(&ontology2, OwlProfile::RL);
       
       assert!(!result2.conforms, "Ontology with ObjectMaxCardinality(2) should not conform to RL profile");
   }
   ```

2. **Fix test_rl_profile_superclass_restrictions**:
   ```rust
   #[test]
   fn test_rl_profile_superclass_restrictions() {
       // Test RL superclass expression support
       let ontology_str = r#"Ontology(<http://example.com/ontology>
         SubClassOf(
           Class(<http://example.com/Student>)
           ObjectIntersectionOf(
             Class(<http://example.com/Person>)
             ObjectComplementOf(Class(<http://example.com/Employee>))
             ObjectAllValuesFrom(ObjectProperty(<http://example.com/worksFor>) Class(<http://example.com/Organization>))
             ObjectHasValue(ObjectProperty(<http://example.com/hasName>) NamedIndividual(<http://example.com/john>))
             ObjectMaxCardinality(1 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>))
           )
         )
       )"#;
       
       let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
       let result = check_profile_compliance(&ontology, OwlProfile::RL);
       
       assert!(result.conforms, "RL superclass expressions should conform to RL profile. Violations: {:?}", result.violations);
   }
   ```

### Task 4.2: Add Additional RL Test Cases

Add more comprehensive RL test cases to ensure full coverage:

```rust
#[test]
fn test_rl_profile_has_key_axiom() {
    // Test HasKey axiom support in RL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      HasKey(Class(<http://example.com/Person>) ObjectProperty(<http://example.com/hasSSN>) DataProperty(<http://example.com/hasSSN>))
    )"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "HasKey axiom should conform to RL profile. Violations: {:?}", result.violations);
}

#[test]
fn test_rl_profile_object_has_self() {
    // Test ObjectHasSelf support in RL
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Narcissist>) ObjectHasSelf(ObjectProperty(<http://example.com/loves>)))
    )"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "ObjectHasSelf should conform to RL profile. Violations: {:?}", result.violations);
}

#[test]
fn test_rl_profile_min_cardinality() {
    // Test ObjectMinCardinality support in RL (only min 0 or 1)
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) ObjectMinCardinality(1 ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>)))
    )"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "ObjectMinCardinality(1) should conform to RL profile. Violations: {:?}", result.violations);
    
    // Test with min 2 (should violate RL)
    let ontology_str2 = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) ObjectMinCardinality(2 ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>)))
    )"#;
    
    let ontology2 = load_ontology(ontology_str2).expect("Failed to parse ontology");
    let result2 = check_profile_compliance(&ontology2, OwlProfile::RL);
    
    assert!(!result2.conforms, "ObjectMinCardinality(2) should not conform to RL profile");
}

#[test]
fn test_rl_profile_exact_cardinality() {
    // Test ObjectExactCardinality support in RL (only exact 0 or 1)
    let ontology_str = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) ObjectExactCardinality(1 ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>)))
    )"#;
    
    let ontology = load_ontology(ontology_str).expect("Failed to parse ontology");
    let result = check_profile_compliance(&ontology, OwlProfile::RL);
    
    assert!(result.conforms, "ObjectExactCardinality(1) should conform to RL profile. Violations: {:?}", result.violations);
    
    // Test with exact 2 (should violate RL)
    let ontology_str2 = r#"Ontology(<http://example.com/ontology>
      SubClassOf(Class(<http://example.com/Student>) ObjectExactCardinality(2 ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>)))
    )"#;
    
    let ontology2 = load_ontology(ontology_str2).expect("Failed to parse ontology");
    let result2 = check_profile_compliance(&ontology2, OwlProfile::RL);
    
    assert!(!result2.conforms, "ObjectExactCardinality(2) should not conform to RL profile");
}
```

## Implementation Timeline

### Week 1: Parser Fixes
- **Days 1-2**: Fix cardinality expression parsing (ParseIntError)
- **Days 3-4**: Add missing enum variants (HasKey, ObjectHasSelf)
- **Days 5-7**: Test parser fixes with existing RL tests

### Week 2: RL Validation Logic
- **Days 1-3**: Implement complete RL class expression validation
- **Days 4-5**: Implement RL axiom validation
- **Days 6-7**: Test validation logic with comprehensive test cases

### Week 3: Testing and Validation
- **Days 1-2**: Fix failing RL tests
- **Days 3-4**: Add additional RL test cases
- **Days 5-7**: Validate against OWL 2 RL specification

## Success Metrics

1. ✅ All 7 RL profile tests pass
2. ✅ No regressions in EL or QL profile functionality
3. ✅ Clean, well-documented implementation
4. ✅ Comprehensive test coverage
5. ✅ Proper error handling and reporting

## Conclusion

This detailed implementation plan provides a clear roadmap for completing the RL profile implementation. By addressing the parser issues and implementing proper validation logic, the owl2_rs library will have complete OWL 2 profile support that aligns with the W3C OWL 2 specification.