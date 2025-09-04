//! # OWL 2 Profile Checker
//!
//! This module provides functionality to check if an ontology conforms to
//! specific OWL 2 profiles (EL, QL, RL).

use crate::{
    Axiom, ClassAxiom, ObjectPropertyAxiom, DataPropertyAxiom, 
    Assertion, ClassExpression, ObjectPropertyExpression,
    Ontology, DataRange
};

/// Represents the OWL 2 profiles
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OwlProfile {
    /// OWL 2 EL profile
    EL,
    /// OWL 2 QL profile
    QL,
    /// OWL 2 RL profile
    RL,
    /// Full OWL 2
    Full,
}

/// Result of profile checking
#[derive(Debug, Clone)]
pub struct ProfileCheckResult {
    /// The profile that was checked
    pub profile: OwlProfile,
    /// Whether the ontology conforms to the profile
    pub conforms: bool,
    /// Reasons why the ontology doesn't conform (if it doesn't)
    pub violations: Vec<String>,
}

/// Checks if an ontology conforms to a specific OWL 2 profile
pub fn check_profile_compliance(ontology: &Ontology, profile: OwlProfile) -> ProfileCheckResult {
    let mut violations = Vec::new();
    
    match profile {
        OwlProfile::EL => {
            check_el_profile(ontology, &mut violations);
        },
        OwlProfile::QL => {
            check_ql_profile(ontology, &mut violations);
        },
        OwlProfile::RL => {
            check_rl_profile(ontology, &mut violations);
        },
        OwlProfile::Full => {
            // Full OWL 2 allows everything, so no violations
        },
    }
    
    ProfileCheckResult {
        profile,
        conforms: violations.is_empty(),
        violations,
    }
}

/// Checks EL profile compliance
fn check_el_profile(ontology: &Ontology, violations: &mut Vec<String>) {
    for axiom in &ontology.axioms {
        match axiom {
            Axiom::Class(class_axiom) => {
                check_el_class_axiom(class_axiom, violations);
            },
            Axiom::ObjectProperty(op_axiom) => {
                check_el_object_property_axiom(op_axiom, violations);
            },
            Axiom::DataProperty(dp_axiom) => {
                check_el_data_property_axiom(dp_axiom, violations);
            },
            Axiom::Assertion(assertion) => {
                check_el_assertion(assertion, violations);
            },
        }
    }
}

/// Checks if a class axiom is EL-compliant
fn check_el_class_axiom(axiom: &ClassAxiom, violations: &mut Vec<String>) {
    match axiom {
        ClassAxiom::SubClassOf { sub_class, super_class } => {
            if !is_el_class_expression(sub_class) {
                violations.push("SubClassOf axiom has non-EL subclass expression".to_string());
            }
            if !is_el_class_expression(super_class) {
                violations.push("SubClassOf axiom has non-EL superclass expression".to_string());
            }
        },
        ClassAxiom::EquivalentClasses { classes } => {
            for class_expr in classes {
                if !is_el_class_expression(class_expr) {
                    violations.push("EquivalentClasses axiom has non-EL class expression".to_string());
                }
            }
        },
        ClassAxiom::DisjointClasses { classes } => {
            for class_expr in classes {
                if !is_el_class_expression(class_expr) {
                    violations.push("DisjointClasses axiom has non-EL class expression".to_string());
                }
            }
        },
        ClassAxiom::DisjointUnion { class: _, disjoint_classes } => {
            for class_expr in disjoint_classes {
                if !is_el_class_expression(class_expr) {
                    violations.push("DisjointUnion axiom has non-EL class expression".to_string());
                }
            }
        },
    }
}

/// Checks if an object property axiom is EL-compliant
fn check_el_object_property_axiom(axiom: &ObjectPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        ObjectPropertyAxiom::SubObjectPropertyOf { sub_property, super_property } => {
            if !is_el_object_property_expression(sub_property) {
                violations.push("SubObjectPropertyOf axiom has non-EL sub-property expression".to_string());
            }
            if !is_el_object_property_expression(super_property) {
                violations.push("SubObjectPropertyOf axiom has non-EL super-property expression".to_string());
            }
        },
        ObjectPropertyAxiom::EquivalentObjectProperties { properties } => {
            for prop in properties {
                if !is_el_object_property_expression(prop) {
                    violations.push("EquivalentObjectProperties axiom has non-EL property expression".to_string());
                }
            }
        },
        ObjectPropertyAxiom::DisjointObjectProperties { properties } => {
            for prop in properties {
                if !is_el_object_property_expression(prop) {
                    violations.push("DisjointObjectProperties axiom has non-EL property expression".to_string());
                }
            }
        },
        ObjectPropertyAxiom::InverseObjectProperties { prop1, prop2 } => {
            if !is_el_object_property_expression(prop1) {
                violations.push("InverseObjectProperties axiom has non-EL property expression (first)".to_string());
            }
            if !is_el_object_property_expression(prop2) {
                violations.push("InverseObjectProperties axiom has non-EL property expression (second)".to_string());
            }
        },
        ObjectPropertyAxiom::ObjectPropertyDomain { property, domain } => {
            if !is_el_object_property_expression(property) {
                violations.push("ObjectPropertyDomain axiom has non-EL property expression".to_string());
            }
            if !is_el_class_expression(domain) {
                violations.push("ObjectPropertyDomain axiom has non-EL domain expression".to_string());
            }
        },
        ObjectPropertyAxiom::ObjectPropertyRange { property, range } => {
            if !is_el_object_property_expression(property) {
                violations.push("ObjectPropertyRange axiom has non-EL property expression".to_string());
            }
            if !is_el_class_expression(range) {
                violations.push("ObjectPropertyRange axiom has non-EL range expression".to_string());
            }
        },
        ObjectPropertyAxiom::FunctionalObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("FunctionalObjectProperty axiom has non-EL property expression".to_string());
            }
        },
        ObjectPropertyAxiom::InverseFunctionalObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("InverseFunctionalObjectProperty axiom has non-EL property expression".to_string());
            }
        },
        ObjectPropertyAxiom::ReflexiveObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("ReflexiveObjectProperty axiom has non-EL property expression".to_string());
            }
        },
        ObjectPropertyAxiom::IrreflexiveObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("IrreflexiveObjectProperty axiom has non-EL property expression".to_string());
            }
        },
        ObjectPropertyAxiom::SymmetricObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("SymmetricObjectProperty axiom has non-EL property expression".to_string());
            }
        },
        ObjectPropertyAxiom::AsymmetricObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("AsymmetricObjectProperty axiom has non-EL property expression".to_string());
            }
        },
        ObjectPropertyAxiom::TransitiveObjectProperty { property } => {
            if !is_el_object_property_expression(property) {
                violations.push("TransitiveObjectProperty axiom has non-EL property expression".to_string());
            }
        },
    }
}

/// Checks if a data property axiom is EL-compliant
fn check_el_data_property_axiom(axiom: &DataPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        DataPropertyAxiom::SubDataPropertyOf { sub_property: _, super_property: _ } => {
            // All sub-data-property axioms are EL-compliant
        },
        DataPropertyAxiom::EquivalentDataProperties { properties: _ } => {
            // All equivalent data properties axioms are EL-compliant
        },
        DataPropertyAxiom::DisjointDataProperties { properties: _ } => {
            // All disjoint data properties axioms are EL-compliant
        },
        DataPropertyAxiom::DataPropertyDomain { property: _, domain } => {
            if !is_el_class_expression(domain) {
                violations.push("DataPropertyDomain axiom has non-EL domain expression".to_string());
            }
        },
        DataPropertyAxiom::DataPropertyRange { property: _, range } => {
            // Data property ranges in EL are restricted to datatypes
            match range {
                crate::DataRange::Datatype(_) => {
                    // Datatypes are EL-compliant
                },
                _ => {
                    violations.push("DataPropertyRange axiom has non-EL range expression".to_string());
                }
            }
        },
        DataPropertyAxiom::FunctionalDataProperty { property: _ } => {
            // All functional data property axioms are EL-compliant
        },
    }
}

/// Checks if an assertion is EL-compliant
fn check_el_assertion(assertion: &Assertion, violations: &mut Vec<String>) {
    match assertion {
        Assertion::SameIndividual { individuals: _ } => {
            // All same individual assertions are EL-compliant
        },
        Assertion::DifferentIndividuals { individuals: _ } => {
            // All different individual assertions are EL-compliant
        },
        Assertion::ClassAssertion { class, individual: _ } => {
            if !is_el_class_expression(class) {
                violations.push("ClassAssertion has non-EL class expression".to_string());
            }
        },
        Assertion::ObjectPropertyAssertion { property, source: _, target: _ } => {
            if !is_el_object_property_expression(property) {
                violations.push("ObjectPropertyAssertion has non-EL property expression".to_string());
            }
        },
        Assertion::DataPropertyAssertion { property: _, source: _, target: _ } => {
            // All data property assertions are EL-compliant
        },
        Assertion::NegativeObjectPropertyAssertion { property, source: _, target: _ } => {
            if !is_el_object_property_expression(property) {
                violations.push("NegativeObjectPropertyAssertion has non-EL property expression".to_string());
            }
        },
        Assertion::NegativeDataPropertyAssertion { property: _, source: _, target: _ } => {
            // All negative data property assertions are EL-compliant
        },
    }
}

/// Checks if a class expression is EL-compliant
fn is_el_class_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            // Intersections are EL-compliant if all sub-expressions are EL-compliant
            sub_exprs.iter().all(|sub_expr| is_el_class_expression(sub_expr))
        },
        ClassExpression::ObjectSomeValuesFrom { property: _, filler } => {
            // Some values from is EL-compliant if filler is EL-compliant
            is_el_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property: _, value: _ } => {
            // Has value is EL-compliant
            true
        },
        // All other class expressions are not EL-compliant
        _ => false,
    }
}

/// Checks if an object property expression is EL-compliant
fn is_el_object_property_expression(expr: &ObjectPropertyExpression) -> bool {
    match expr {
        ObjectPropertyExpression::ObjectProperty(_) => true,
        ObjectPropertyExpression::InverseObjectProperty(_) => true,
        // Property chains are not EL-compliant
        ObjectPropertyExpression::ObjectPropertyChain(_) => false,
    }
}

/// Checks QL profile compliance
fn check_ql_profile(ontology: &Ontology, violations: &mut Vec<String>) {
    for axiom in &ontology.axioms {
        match axiom {
            Axiom::Class(class_axiom) => {
                check_ql_class_axiom(class_axiom, violations);
            },
            Axiom::ObjectProperty(op_axiom) => {
                check_ql_object_property_axiom(op_axiom, violations);
            },
            Axiom::DataProperty(dp_axiom) => {
                check_ql_data_property_axiom(dp_axiom, violations);
            },
            Axiom::Assertion(assertion) => {
                check_ql_assertion(assertion, violations);
            },
        }
    }
}

/// Checks if a class axiom is QL-compliant
fn check_ql_class_axiom(axiom: &ClassAxiom, violations: &mut Vec<String>) {
    match axiom {
        ClassAxiom::SubClassOf { sub_class, super_class } => {
            if !is_ql_subclass_expression(sub_class) {
                violations.push("SubClassOf axiom has non-QL subclass expression".to_string());
            }
            if !is_ql_superclass_expression(super_class) {
                violations.push("SubClassOf axiom has non-QL superclass expression".to_string());
            }
        },
        ClassAxiom::EquivalentClasses { classes } => {
            for class_expr in classes {
                // In QL, equivalent classes can use any valid class expression
                if !is_ql_valid_class_expression(class_expr) {
                    violations.push("EquivalentClasses axiom has non-QL class expression".to_string());
                }
            }
        },
        ClassAxiom::DisjointClasses { classes } => {
            for class_expr in classes {
                // In QL, disjoint classes can use any valid class expression
                if !is_ql_valid_class_expression(class_expr) {
                    violations.push("DisjointClasses axiom has non-QL class expression".to_string());
                }
            }
        },
        ClassAxiom::DisjointUnion { class: _, disjoint_classes: _ } => {
            // DisjointUnion is not allowed in QL
            violations.push("DisjointUnion axiom is not allowed in QL profile".to_string());
        },
    }
}

/// Checks if an object property axiom is QL-compliant
fn check_ql_object_property_axiom(axiom: &ObjectPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        ObjectPropertyAxiom::SubObjectPropertyOf { sub_property, super_property } => {
            // QL does not allow property chains in SubObjectPropertyOf
            if let ObjectPropertyExpression::ObjectPropertyChain(_) = sub_property {
                violations.push("SubObjectPropertyOf with property chain is not allowed in QL profile".to_string());
            }
            if let ObjectPropertyExpression::ObjectPropertyChain(_) = super_property {
                violations.push("SubObjectPropertyOf with property chain is not allowed in QL profile".to_string());
            }
        },
        ObjectPropertyAxiom::TransitiveObjectProperty { property: _ } => {
            // TransitiveObjectProperty is not allowed in QL
            violations.push("TransitiveObjectProperty axiom is not allowed in QL profile".to_string());
        },
        ObjectPropertyAxiom::FunctionalObjectProperty { property: _ } => {
            // FunctionalObjectProperty is not allowed in QL
            violations.push("FunctionalObjectProperty axiom is not allowed in QL profile".to_string());
        },
        ObjectPropertyAxiom::InverseFunctionalObjectProperty { property: _ } => {
            // InverseFunctionalObjectProperty is not allowed in QL
            violations.push("InverseFunctionalObjectProperty axiom is not allowed in QL profile".to_string());
        },
        // All other object property axioms are allowed in QL
        _ => {},
    }
}

/// Checks if a data property axiom is QL-compliant
fn check_ql_data_property_axiom(axiom: &DataPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        DataPropertyAxiom::FunctionalDataProperty { property: _ } => {
            // FunctionalDataProperty is not allowed in QL
            violations.push("FunctionalDataProperty axiom is not allowed in QL profile".to_string());
        },
        // All other data property axioms are allowed in QL
        _ => {},
    }
}

/// Checks if an assertion is QL-compliant
fn check_ql_assertion(assertion: &Assertion, violations: &mut Vec<String>) {
    match assertion {
        Assertion::SameIndividual { individuals: _ } => {
            // SameIndividual assertions are not allowed in QL
            violations.push("SameIndividual assertion is not allowed in QL profile".to_string());
        },
        Assertion::NegativeObjectPropertyAssertion { property: _, source: _, target: _ } => {
            // NegativeObjectPropertyAssertion is not allowed in QL
            violations.push("NegativeObjectPropertyAssertion is not allowed in QL profile".to_string());
        },
        Assertion::NegativeDataPropertyAssertion { property: _, source: _, target: _ } => {
            // NegativeDataPropertyAssertion is not allowed in QL
            violations.push("NegativeDataPropertyAssertion is not allowed in QL profile".to_string());
        },
        // All other assertions are allowed in QL
        _ => {},
    }
}

/// Checks if a class expression is valid as a subclass in QL
fn is_ql_subclass_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        // All other class expressions are not allowed in subclass position in QL
        _ => false,
    }
}

/// Checks if a class expression is valid as a superclass in QL
fn is_ql_superclass_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            // Intersections are allowed in superclass position
            sub_exprs.iter().all(|sub_expr| is_ql_superclass_expression(sub_expr))
        },
        ClassExpression::ObjectComplementOf(sub_expr) => {
            // Complement is allowed in superclass position
            is_ql_valid_class_expression(sub_expr)
        },
        ClassExpression::ObjectSomeValuesFrom { property: _, filler } => {
            // ObjectSomeValuesFrom is allowed in superclass position
            is_ql_valid_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property: _, value: _ } => {
            // ObjectHasValue is allowed in superclass position
            true
        },
        // All other class expressions are not allowed in superclass position in QL
        _ => false,
    }
}

/// Checks if a class expression is valid in QL (general)
fn is_ql_valid_class_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            sub_exprs.iter().all(|sub_expr| is_ql_valid_class_expression(sub_expr))
        },
        ClassExpression::ObjectComplementOf(sub_expr) => {
            is_ql_valid_class_expression(sub_expr)
        },
        ClassExpression::ObjectSomeValuesFrom { property: _, filler } => {
            is_ql_valid_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property: _, value: _ } => true,
        // All other class expressions are not allowed in QL
        _ => false,
    }
}

/// Checks if a data range is valid in QL
fn is_ql_valid_data_range(range: &crate::DataRange) -> bool {
    match range {
        crate::DataRange::Datatype(_) => true,
        crate::DataRange::DataIntersectionOf(sub_ranges) => {
            sub_ranges.iter().all(|sub_range| is_ql_valid_data_range(sub_range))
        },
        // All other data ranges are not allowed in QL
        _ => false,
    }
}

/// Checks if a class expression is valid in RL
fn is_rl_valid_class_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            sub_exprs.iter().all(|sub_expr| is_rl_valid_class_expression(sub_expr))
        },
        ClassExpression::ObjectUnionOf(sub_exprs) => {
            sub_exprs.iter().all(|sub_expr| is_rl_valid_class_expression(sub_expr))
        },
        ClassExpression::ObjectComplementOf(sub_expr) => {
            is_rl_valid_class_expression(sub_expr)
        },
        ClassExpression::ObjectOneOf(individuals) => {
            !individuals.is_empty()
        },
        ClassExpression::ObjectSomeValuesFrom { property: _, filler } => {
            is_rl_valid_class_expression(filler)
        },
        ClassExpression::ObjectAllValuesFrom { property: _, filler } => {
            is_rl_valid_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property: _, value: _ } => true,
        ClassExpression::ObjectHasSelf(_) => true,
        ClassExpression::ObjectMinCardinality { min, property: _, filler } => {
            // Only min 0 or 1 allowed in RL
            *min <= 1 && filler.as_ref().map_or(true, |f| is_rl_valid_class_expression(f))
        },
        ClassExpression::ObjectMaxCardinality { max, property: _, filler } => {
            // Only max 0 or 1 allowed in RL
            *max <= 1 && filler.as_ref().map_or(true, |f| is_rl_valid_class_expression(f))
        },
        ClassExpression::ObjectExactCardinality { cardinality, property: _, filler } => {
            // Only exact 0 or 1 allowed in RL
            *cardinality <= 1 && filler.as_ref().map_or(true, |f| is_rl_valid_class_expression(f))
        },
        // All other class expressions are not allowed in RL
        _ => false,
    }
}

/// Checks RL profile compliance
fn check_rl_profile(ontology: &Ontology, violations: &mut Vec<String>) {
    for axiom in &ontology.axioms {
        match axiom {
            Axiom::Class(class_axiom) => {
                check_rl_class_axiom(class_axiom, violations);
            },
            Axiom::ObjectProperty(op_axiom) => {
                check_rl_object_property_axiom(op_axiom, violations);
            },
            Axiom::DataProperty(dp_axiom) => {
                check_rl_data_property_axiom(dp_axiom, violations);
            },
            Axiom::Assertion(assertion) => {
                check_rl_assertion(assertion, violations);
            },
        }
    }
}

/// Checks if a class axiom is RL-compliant
fn check_rl_class_axiom(axiom: &ClassAxiom, violations: &mut Vec<String>) {
    match axiom {
        ClassAxiom::SubClassOf { sub_class, super_class } => {
            if !is_rl_subclass_expression(sub_class) {
                violations.push("SubClassOf axiom has non-RL subclass expression".to_string());
            }
            if !is_rl_superclass_expression(super_class) {
                violations.push("SubClassOf axiom has non-RL superclass expression".to_string());
            }
        },
        ClassAxiom::EquivalentClasses { classes } => {
            for class_expr in classes {
                if !is_rl_equivalent_expression(class_expr) {
                    violations.push("EquivalentClasses axiom has non-RL class expression".to_string());
                }
            }
        },
        ClassAxiom::DisjointClasses { classes } => {
            for class_expr in classes {
                // DisjointClasses can use any valid RL class expression
                if !is_rl_valid_class_expression(class_expr) {
                    violations.push("DisjointClasses axiom has non-RL class expression".to_string());
                }
            }
        },
        ClassAxiom::DisjointUnion { class: _, disjoint_classes: _ } => {
            // DisjointUnion is not allowed in RL
            violations.push("DisjointUnion axiom is not allowed in RL profile".to_string());
        },
    }
}

/// Checks if an object property axiom is RL-compliant
fn check_rl_object_property_axiom(axiom: &ObjectPropertyAxiom, violations: &mut Vec<String>) {
    match axiom {
        ObjectPropertyAxiom::ReflexiveObjectProperty { property: _ } => {
            // ReflexiveObjectProperty is not allowed in RL
            violations.push("ReflexiveObjectProperty axiom is not allowed in RL profile".to_string());
        },
        // All other object property axioms are allowed in RL
        _ => {},
    }
}

/// Checks if a data property axiom is RL-compliant
fn check_rl_data_property_axiom(_axiom: &DataPropertyAxiom, _violations: &mut Vec<String>) {
    // All data property axioms are allowed in RL
    // Note: We might want to add datatype restrictions for owl:real and owl:rational
}

/// Checks if an assertion is RL-compliant
fn check_rl_assertion(assertion: &Assertion, violations: &mut Vec<String>) {
    match assertion {
        Assertion::ClassAssertion { class, individual: _ } => {
            // Class assertions in RL are restricted to superclass expressions
            if !is_rl_superclass_expression(class) {
                violations.push("ClassAssertion has non-RL class expression".to_string());
            }
        },
        Assertion::HasKey { class: _, object_property_expression: _, data_property: _ } => {
            // HasKey is allowed in RL but with restrictions
            // For now, we'll allow it but note that a full implementation would check the restrictions
        },
        // All other assertions are allowed in RL
        _ => {},
    }
}

/// Checks if a class expression is RL-compliant in subclass position
fn is_rl_subclass_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            // Intersections are RL-compliant if all sub-expressions are RL-compliant
            sub_exprs.iter().all(|sub_expr| is_rl_subclass_expression(sub_expr))
        },
        ClassExpression::ObjectUnionOf(sub_exprs) => {
            // Unions are RL-compliant if all sub-expressions are RL-compliant
            sub_exprs.iter().all(|sub_expr| is_rl_subclass_expression(sub_expr))
        },
        ClassExpression::ObjectOneOf(individuals) => {
            // Enumerations are RL-compliant
            !individuals.is_empty()
        },
        ClassExpression::ObjectSomeValuesFrom { property, filler } => {
            // Some values from is RL-compliant if property and filler are RL-compliant
            is_rl_object_property_expression(property) && is_rl_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property, value: _ } => {
            // Has value is RL-compliant if property is RL-compliant
            is_rl_object_property_expression(property)
        },
        // All other class expressions are not RL-compliant in subclass position
        _ => false,
    }
}

/// Checks if a class expression is RL-compliant in superclass position
fn is_rl_superclass_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            // Intersections are RL-compliant if all sub-expressions are RL-compliant
            sub_exprs.iter().all(|sub_expr| is_rl_superclass_expression(sub_expr))
        },
        ClassExpression::ObjectComplementOf(sub_expr) => {
            // Complement is RL-compliant if sub-expression is RL-compliant
            is_rl_class_expression(sub_expr)
        },
        ClassExpression::ObjectSomeValuesFrom { property, filler } => {
            // Some values from is RL-compliant if property and filler are RL-compliant
            is_rl_object_property_expression(property) && is_rl_class_expression(filler)
        },
        ClassExpression::ObjectAllValuesFrom { property, filler } => {
            // All values from is RL-compliant if property and filler are RL-compliant
            is_rl_object_property_expression(property) && is_rl_class_expression(filler)
        },
        ClassExpression::ObjectHasValue { property, value: _ } => {
            // Has value is RL-compliant if property is RL-compliant
            is_rl_object_property_expression(property)
        },
        ClassExpression::ObjectMaxCardinality { max, property, filler } => {
            // Only max 0 or 1 allowed in RL
            *max <= 1 && is_rl_object_property_expression(property) && 
            filler.as_ref().map_or(true, |f| is_rl_class_expression(f))
        },
        // All other class expressions are not RL-compliant in superclass position
        _ => false,
    }
}

/// Checks if a class expression is RL-compliant in equivalent position
fn is_rl_equivalent_expression(expr: &ClassExpression) -> bool {
    match expr {
        ClassExpression::Class(_) => true,
        ClassExpression::ObjectIntersectionOf(sub_exprs) => {
            // Intersections are RL-compliant if all sub-expressions are RL-compliant
            sub_exprs.iter().all(|sub_expr| is_rl_equivalent_expression(sub_expr))
        },
        ClassExpression::ObjectHasValue { property, value: _ } => {
            // Has value is RL-compliant if property is RL-compliant
            is_rl_object_property_expression(property)
        },
        // All other class expressions are not RL-compliant in equivalent position
        _ => false,
    }
}

/// Checks if a class expression is RL-compliant (general)
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
            *min <= 1 && is_rl_object_property_expression(property) && 
            filler.as_ref().map_or(true, |f| is_rl_class_expression(f))
        },
        ClassExpression::ObjectMaxCardinality { max, property, filler } => {
            // Only max 0 or 1 allowed in RL
            *max <= 1 && is_rl_object_property_expression(property) && 
            filler.as_ref().map_or(true, |f| is_rl_class_expression(f))
        },
        ClassExpression::ObjectExactCardinality { cardinality, property, filler } => {
            // Only exact 0 or 1 allowed in RL
            *cardinality <= 1 && is_rl_object_property_expression(property) && 
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

/// Checks if a data range is RL-compliant
fn is_rl_data_range(range: &DataRange) -> bool {
    match range {
        DataRange::Datatype(datatype) => {
            // RL does not support owl:real and owl:rational
            let iri = &datatype.0.0;
            !iri.contains("owl:real") && !iri.contains("owl:rational")
        },
        DataRange::DataIntersectionOf(sub_ranges) => {
            sub_ranges.iter().all(|sub_range| is_rl_data_range(sub_range))
        },
        // All other data ranges are not RL-compliant
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::load_ontology;

    #[test]
    fn test_el_profile_checker() {
        // Simple EL ontology
        let el_ontology_str = r#"Ontology(<http://example.com/ontology>
          SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
          
          ObjectPropertyDomain(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
          ObjectPropertyRange(ObjectProperty(<http://example.com/hasParent>) Class(<http://example.com/Person>))
          
          ClassAssertion(Class(<http://example.com/Student>) NamedIndividual(<http://example.com/john>))
        )"#;
        
        let ontology = load_ontology(el_ontology_str).expect("Failed to parse ontology");
        let result = check_profile_compliance(&ontology, OwlProfile::EL);
        
        assert!(result.conforms);
        assert!(result.violations.is_empty());
    }

    #[test]
    fn test_non_el_profile_checker() {
        // Ontology with union (not EL-compliant)
        let full_ontology_str = r#"Ontology(<http://example.com/ontology>
  SubClassOf(ObjectUnionOf(Class(<http://example.com/Student>) Class(<http://example.com/Employee>)) Class(<http://example.com/Person>))
)"#;
        
        let ontology = load_ontology(full_ontology_str).expect("Failed to parse ontology");
        let result = check_profile_compliance(&ontology, OwlProfile::EL);
        
        assert!(!result.conforms);
        assert!(!result.violations.is_empty());
    }
}