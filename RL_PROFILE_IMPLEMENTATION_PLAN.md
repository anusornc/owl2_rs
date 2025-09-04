# RL Profile Implementation Plan

## Current Status Analysis

The RL profile implementation has basic functionality but faces parsing issues with cardinality expressions. Let me analyze the specific problems:

### Identified Issues:
1. **ParseIntError with ObjectMaxCardinality** - Parser fails to correctly extract numeric values
2. **Grammar issues with cardinality expressions** - Incorrect tokenization or parsing rules
3. **Incomplete RL validation logic** - Some validation rules need refinement

## Detailed Implementation Plan

### Phase 1: Parser Fixes (Week 1)

#### Task 1.1: Debug Cardinality Expression Parsing
- **Issue**: ParseIntError when parsing ObjectMaxCardinality expressions
- **Root Cause**: Likely incorrect tokenization or extraction of numeric values
- **Solution**: 
  1. Examine grammar rules for cardinality expressions
  2. Debug parsing logic for numeric value extraction
  3. Fix tokenization issues with cardinality expressions

#### Task 1.2: Fix Object Property Chain Parsing
- **Issue**: Missing object_property_chain rule in grammar
- **Solution**: Add proper grammar rule for object property chains

#### Task 1.3: Fix HasKey Assertion Parsing
- **Issue**: Missing HasKey variant in Assertion enum
- **Solution**: Add HasKey variant to Assertion enum

### Phase 2: RL Validation Logic Enhancement (Week 2)

#### Task 2.1: Complete RL Class Expression Validation
- **Issue**: Incomplete validation logic for RL class expressions
- **Solution**: 
  1. Implement complete RL class expression validation
  2. Add support for ObjectHasSelf expressions
  3. Refine cardinality restriction validation (max 0 or 1)

#### Task 2.2: Complete RL Axiom Validation
- **Issue**: Missing validation for RL-specific axioms
- **Solution**:
  1. Add validation for ReflexiveObjectProperty (should reject)
  2. Add validation for DisjointUnion (should reject)
  3. Add validation for functional object properties in RL context

#### Task 2.3: Complete RL Data Range Validation
- **Issue**: Incomplete data range validation for RL
- **Solution**: 
  1. Add proper validation for owl:real and owl:rational restrictions
  2. Implement complete data range validation logic

### Phase 3: Testing and Validation (Week 3)

#### Task 3.1: Fix Failing RL Tests
- **Issue**: 2 RL tests currently failing
- **Solution**: 
  1. Debug and fix test_rl_profile_cardinality_restrictions
  2. Debug and fix test_rl_profile_superclass_restrictions

#### Task 3.2: Add Comprehensive RL Test Coverage
- **Issue**: Missing edge case tests
- **Solution**: 
  1. Add tests for complex RL class expressions
  2. Add tests for RL-specific axiom restrictions
  3. Add tests for data range restrictions

#### Task 3.3: Validate Against OWL 2 RL Specification
- **Issue**: Potential specification alignment issues
- **Solution**:
  1. Cross-check implementation with OWL 2 RL specification
  2. Ensure all RL restrictions are properly enforced
  3. Verify edge cases align with specification

## Technical Implementation Details

### 1. Grammar Fixes

#### Current Issue:
The grammar file is missing proper rules for cardinality expressions and object property chains.

#### Solution:
Update `src/grammar.pest` to include:

```pest
object_property_chain = { "ObjectPropertyChain(" ~ object_property_expression* ~ ")" }

object_min_cardinality = { "ObjectMinCardinality(" ~ ASCII_DIGIT+ ~ object_property_expression ~ class_expression? ~ ")" }
object_max_cardinality = { "ObjectMaxCardinality(" ~ ASCII_DIGIT+ ~ object_property_expression ~ class_expression? ~ ")" }
object_exact_cardinality = { "ObjectExactCardinality(" ~ ASCII_DIGIT+ ~ object_property_expression ~ class_expression? ~ ")" }

data_min_cardinality = { "DataMinCardinality(" ~ ASCII_DIGIT+ ~ data_property ~ data_range? ~ ")" }
data_max_cardinality = { "DataMaxCardinality(" ~ ASCII_DIGIT+ ~ data_property ~ data_range? ~ ")" }
data_exact_cardinality = { "DataExactCardinality(" ~ ASCII_DIGIT+ ~ data_property ~ data_range? ~ ")" }
```

### 2. Parser Implementation Fixes

#### Current Issue:
ParseIntError when extracting numeric values from cardinality expressions.

#### Solution:
Fix the parsing logic in `src/parser.rs`:

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
},
```

### 3. Data Structures Updates

#### Current Issue:
Missing HasKey variant in Assertion enum.

#### Solution:
Update `src/lib.rs`:

```rust
pub enum Assertion {
    // ... existing variants ...
    HasKey {
        class: Class,
        object_property_expression: Vec<ObjectPropertyExpression>,
        data_property: Vec<DataProperty>,
    },
}
```

### 4. RL Validation Logic

#### Current Issue:
Incomplete RL validation logic.

#### Solution:
Enhance `src/owl2_profile.rs`:

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
```

## Timeline and Milestones

### Week 1: Parser Fixes
- **Day 1-2**: Debug and fix cardinality expression parsing
- **Day 3-4**: Fix object property chain parsing
- **Day 5**: Fix HasKey assertion parsing
- **Day 6-7**: Test parser fixes with existing RL tests

### Week 2: RL Validation Logic Enhancement
- **Day 1-2**: Complete RL class expression validation
- **Day 3-4**: Complete RL axiom validation
- **Day 5-6**: Complete RL data range validation
- **Day 7**: Integration testing of validation logic

### Week 3: Testing and Validation
- **Day 1-2**: Fix failing RL tests
- **Day 3-4**: Add comprehensive RL test coverage
- **Day 5-6**: Validate against OWL 2 RL specification
- **Day 7**: Final testing and documentation updates

## Risk Mitigation

### 1. Parser Complexity Risk
- **Risk**: Parser fixes may introduce new issues
- **Mitigation**: Thorough testing of all parser changes
- **Mitigation**: Incremental implementation with frequent testing

### 2. Specification Alignment Risk
- **Risk**: RL validation may not fully align with specification
- **Mitigation**: Cross-check with official OWL 2 RL specification
- **Mitigation**: Use standard OWL 2 conformance test cases

### 3. Performance Risk
- **Risk**: RL validation may impact performance
- **Mitigation**: Profile validation logic for performance bottlenecks
- **Mitigation**: Optimize critical validation paths

## Success Criteria

### Technical Criteria
1. ✅ All RL profile tests pass (7/7)
2. ✅ Parser correctly handles all RL constructs
3. ✅ Validation logic aligns with OWL 2 RL specification
4. ✅ No regressions in EL or QL profile functionality

### Quality Criteria
1. ✅ Clean, well-documented implementation
2. ✅ Comprehensive test coverage
3. ✅ No memory leaks or performance issues
4. ✅ Proper error handling and reporting

### Documentation Criteria
1. ✅ Updated API documentation
2. ✅ Updated developer guide
3. ✅ Updated tutorials
4. ✅ Updated testing documentation

## Expected Outcomes

Upon completion of this implementation plan:

1. **Complete RL Profile Support**: The owl2_rs library will have full OWL 2 RL profile checking capabilities
2. **Robust Parser**: The parser will correctly handle all OWL 2 RL constructs
3. **Comprehensive Validation**: The validation logic will enforce all RL profile restrictions
4. **Professional-Quality Implementation**: Clean, well-tested, documented code following Rust best practices
5. **Full Test Coverage**: All RL profile tests will pass, ensuring reliability and correctness

This implementation will make owl2_rs one of the most complete OWL 2 libraries available, with comprehensive support for all three major OWL 2 profiles (EL, QL, RL).