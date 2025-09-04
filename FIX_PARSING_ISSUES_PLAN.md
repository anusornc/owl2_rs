# RL Profile Parsing Issues Fix Plan

## Current Issues

Two RL profile tests are failing due to parsing issues:

1. **test_rl_profile_cardinality_restrictions** - ParseIntError when parsing ObjectMaxCardinality
2. **test_rl_profile_superclass_restrictions** - ParseIntError when parsing ObjectMaxCardinality

## Root Cause Analysis

The error message indicates:
```
ParseIntError { kind: InvalidDigit }
```

This means the parser is trying to parse a string as an integer, but the string contains non-numeric characters.

## Debugging Steps

### Step 1: Examine the Test Cases

Let's look at the failing test cases:

#### test_rl_profile_cardinality_restrictions
```rust
#[test]
fn test_rl_profile_cardinality_restrictions() {
    // Test RL cardinality restrictions (only max 0 or 1 allowed)
    let ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(Class(<http://example.com/Student>) ObjectMaxCardinality(1 ObjectProperty(<http://example.com/hasAdvisor>) Class(<http://example.com/Person>)))
)"#;
```

#### test_rl_profile_superclass_restrictions
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
```

### Step 2: Examine the Grammar

Let's look at the grammar definition for cardinality expressions:

#### Current Grammar (src/grammar.pest)
```pest
object_min_cardinality = { "ObjectMinCardinality(" ~ (ASCII_DIGIT+)+ ~ object_property_expression ~ class_expression? ~ ")" }
object_max_cardinality = { "ObjectMaxCardinality(" ~ (ASCII_DIGIT+)+ ~ object_property_expression ~ class_expression? ~ ")" }
object_exact_cardinality = { "ObjectExactCardinality(" ~ (ASCII_DIGIT+)+ ~ object_property_expression ~ class_expression? ~ ")" }
```

### Step 3: Examine the Parsing Logic

Let's look at the parsing logic in src/parser.rs:

#### Current Parsing Logic
```rust
Rule::object_max_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let max: u32 = inner.next().unwrap().as_str().parse().unwrap();
    let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
    let filler = if let Some(filler_pair) = inner.next() {
        Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str()).unwrap()))
    } else {
        None
    };
    ClassExpression::ObjectMaxCardinality { max, property, filler }
},
```

### Step 4: Identify the Problem

The issue appears to be that `inner.next().unwrap().as_str()` is returning something other than a pure number. This could be caused by:

1. Incorrect grammar rule that includes non-numeric characters
2. Tokenization issue where the number is mixed with other tokens
3. Incorrect parsing order where tokens are not properly separated

## Fix Implementation Plan

### Fix 1: Debug the Grammar and Tokenization

First, let's add some debug output to see what tokens are being parsed:

```rust
Rule::object_max_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let max_token = inner.next().unwrap();
    let max_str = max_token.as_str();
    println!("DEBUG: Parsing ObjectMaxCardinality, max token: '{}'", max_str);
    
    let max: u32 = max_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", max_str, e),
        })
    })?;
    
    let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str()).map_err(Box::new)?))
    } else {
        None
    };
    ClassExpression::ObjectMaxCardinality { max, property, filler }
},
```

### Fix 2: Correct the Grammar Definition

The grammar might need adjustment to properly separate tokens:

```pest
object_max_cardinality = { "ObjectMaxCardinality(" ~ ASCII_DIGIT+ ~ " " ~ object_property_expression ~ (" " ~ class_expression)? ~ ")" }
```

Or alternatively, ensure proper whitespace handling:

```pest
object_max_cardinality = { "ObjectMaxCardinality(" ~ ASCII_DIGIT+ ~ WHITESPACE+ ~ object_property_expression ~ (WHITESPACE+ ~ class_expression)? ~ ")" }
```

### Fix 3: Update All Cardinality Parsing Rules

Apply the same fix to all cardinality rules:

```rust
Rule::object_min_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let min_str = inner.next().unwrap().as_str();
    let min: u32 = min_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", min_str, e),
        })
    })?;
    let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str()).map_err(Box::new)?))
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
    let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str()).map_err(Box::new)?))
    } else {
        None
    };
    ClassExpression::ObjectExactCardinality { cardinality, property, filler }
},

Rule::data_min_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let min_str = inner.next().unwrap().as_str();
    let min: u32 = min_str.parse().map_err(|e| {
        Box::new(pest::error::Error::CustomErrorVariant {
            message: format!("Failed to parse cardinality '{}': {}", min_str, e),
        })
    })?;
    let property = OWLParser::parse_data_property(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(OWLParser::parse_data_range(filler_pair.as_str()).map_err(Box::new)?)
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
    let property = OWLParser::parse_data_property(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(OWLParser::parse_data_range(filler_pair.as_str()).map_err(Box::new)?)
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
    let property = OWLParser::parse_data_property(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let filler = if let Some(filler_pair) = inner.next() {
        Some(OWLParser::parse_data_range(filler_pair.as_str()).map_err(Box::new)?)
    } else {
        None
    };
    ClassExpression::DataExactCardinality { cardinality, property, filler }
}
```

### Fix 4: Add HasKey Variant to Assertion Enum

Update src/lib.rs to add the missing HasKey variant:

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

### Fix 5: Update Assertion Parsing Logic

Update src/parser.rs to handle HasKey assertions:

```rust
Rule::has_key => {
    let mut inner = inner_rule_pair.into_inner();
    let class = OWLParser::parse_class(inner.next().unwrap().as_str()).map_err(Box::new)?;
    let object_property_expressions: Vec<ObjectPropertyExpression> = inner.by_ref()
        .take_while(|pair| pair.as_rule() == Rule::object_property_expression)
        .map(|p| OWLParser::parse_object_property_expression(p.as_str()).map_err(Box::new))
        .collect::<Result<Vec<_>, _>>()?;
    let data_properties: Vec<DataProperty> = inner
        .map(|p| OWLParser::parse_data_property(p.as_str()).map_err(Box::new))
        .collect::<Result<Vec<_>, _>>()?;
    Assertion::HasKey {
        class,
        object_property_expression: object_property_expressions,
        data_property: data_properties,
    }
},
```

## Testing Strategy

### Step 1: Add Debug Output
Temporarily add debug output to see what tokens are being parsed:

```rust
Rule::object_max_cardinality => {
    let mut inner = inner_rule_pair.into_inner();
    let max_token = inner.next().unwrap();
    let max_str = max_token.as_str();
    println!("DEBUG: ObjectMaxCardinality max token: '{}'", max_str);
    
    // Continue with parsing...
}
```

### Step 2: Fix and Test Incrementally
1. Fix one cardinality parsing rule
2. Test with one failing test
3. Repeat for all cardinality rules
4. Test all RL profile tests

### Step 3: Validate Against Specification
Cross-check implementation with OWL 2 RL specification to ensure correctness.

## Timeline

### Day 1: Debug and Fix Grammar
- Add debug output to see what tokens are being parsed
- Identify the exact cause of the ParseIntError
- Fix the grammar definition if needed

### Day 2: Fix Parsing Logic
- Update all cardinality parsing rules to properly handle tokenization
- Add proper error handling with descriptive error messages
- Test with individual failing tests

### Day 3: Add Missing Enum Variants
- Add HasKey variant to Assertion enum
- Update parsing logic to handle HasKey assertions
- Test with comprehensive RL tests

### Day 4: Comprehensive Testing
- Run all RL profile tests
- Validate against OWL 2 RL specification
- Fix any remaining issues

### Day 5: Integration Testing
- Run all profile tests (EL, QL, RL)
- Ensure no regressions in other profiles
- Final validation and cleanup

## Success Criteria

1. ✅ All RL profile tests pass (7/7)
2. ✅ No regressions in EL or QL profile functionality
3. ✅ Clean, well-documented implementation
4. ✅ Proper error handling and reporting
5. ✅ Alignment with OWL 2 RL specification

## Expected Outcome

With these fixes implemented, the owl2_rs library will have complete OWL 2 profile checking capabilities for all three profiles (EL, QL, RL) that align with the W3C OWL 2 specification.