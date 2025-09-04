//! # owl2_rs
//!
//! A Rust library for the Web Ontology Language (OWL 2).
//!
//! ## Overview
//!
//! The owl2_rs library provides functionality for working with OWL 2 ontologies,
//! including parsing, consistency checking, classification, and realization.
//!
//! OWL 2 is a powerful ontology language that allows you to represent knowledge
//! in a machine-readable format. This library provides tools to work with OWL 2
//! ontologies in Rust applications.
//!
//! ## Features
//!
//! - Parse OWL 2 ontologies in Functional-Style Syntax
//! - Check ontology consistency (satisfiability)
//! - Compute class hierarchies (classification)
//! - Realize individuals (find their most specific types)
//! - Instance checking
//! - OWL 2 profile compliance checking (EL, QL, RL)
//!
//! ## Modules
//!
//! - [`api`] - The main public API for the library
//! - [`parser`] - The OWL 2 parser implementation
//! - [`reasoner`] - The tableau-based reasoner implementation
//! - [`owl2_profile`] - OWL 2 profile compliance checking
//!
//! ## Basic Usage
//!
//! ```rust
//! use owl2_rs::api::{load_ontology, Reasoner};
//!
//! // Load an ontology from a string
//! let ontology_str = r#"Ontology(<http://example.com/ontology>
//!   SubClassOf(Class(<http://example.com/Student>) Class(<http://example.com/Person>))
//! )"#;
//!
//! let ontology = load_ontology(ontology_str).unwrap();
//!
//! // Create a reasoner and check consistency
//! let mut reasoner = Reasoner::new(ontology);
//! let is_consistent = reasoner.is_consistent();
//! ```
//!
//! ## Documentation
//!
//! For comprehensive documentation, please see:
//!
//! - [User Guide](https://anusornc.github.io/owl2_rs2/) - For detailed usage instructions
//! - [API Documentation](https://anusornc.github.io/owl2_rs2/doc/owl2_rs/) - For detailed API reference
//! - [Tutorials](https://anusornc.github.io/owl2_rs2/tutorials.html) - For step-by-step examples
//!
//! ## Examples
//!
//! See the `examples/` directory in the repository for complete example programs:
//!
//! - `basic_reasoning.rs` - Basic reasoning functionality
//! - `classification_hierarchy_test.rs` - Testing classification hierarchies
//! - `classification_test.rs` - Classification testing
//! - `comprehensive_reasoning.rs` - Comprehensive reasoning test

pub mod parser;
pub mod reasoner;
pub mod api;
pub mod test_runner;
pub mod owl2_profile;

/// An Internationalized Resource Identifier (IRI).
///
/// IRIs are used throughout OWL 2 to identify entities such as classes,
/// properties, and individuals. This implementation wraps a String to
/// provide strong typing.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::IRI;
///
/// let iri = IRI("http://example.com/MyClass".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct IRI(pub String);

/// A node identifier for anonymous individuals.
///
/// Node IDs are used to identify anonymous individuals in OWL 2 ontologies.
/// They typically have the form _:bn where n is a number.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::NodeID;
///
/// let node_id = NodeID("_:b1".to_string());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct NodeID(pub String);

/// A class in an OWL 2 ontology.
///
/// Classes are used to represent sets of individuals. A class is identified by an IRI.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::{Class, IRI};
///
/// let class = Class(IRI("http://example.com/Student".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Class(pub IRI);

/// A datatype in an OWL 2 ontology.
///
/// Datatypes are used to represent sets of data values such as integers, strings, etc.
/// A datatype is identified by an IRI, typically from the XML Schema namespace.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::{Datatype, IRI};
///
/// let integer_datatype = Datatype(IRI("http://www.w3.org/2001/XMLSchema#integer".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Datatype(pub IRI);

/// An object property in an OWL 2 ontology.
///
/// Object properties are used to represent relationships between individuals.
/// An object property is identified by an IRI.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::{ObjectProperty, IRI};
///
/// let has_part = ObjectProperty(IRI("http://example.com/hasPart".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct ObjectProperty(pub IRI);

/// A data property in an OWL 2 ontology.
///
/// Data properties are used to represent relationships between individuals and data values.
/// A data property is identified by an IRI.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::{DataProperty, IRI};
///
/// let has_age = DataProperty(IRI("http://example.com/hasAge".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct DataProperty(pub IRI);

/// Represents the basic building blocks of an ontology.
///
/// Entities are the fundamental components used to construct OWL 2 ontologies.
///
/// # Variants
///
/// * `Class(Class)` - A class entity.
/// * `Datatype(Datatype)` - A datatype entity.
/// * `ObjectProperty(ObjectProperty)` - An object property entity.
/// * `DataProperty(DataProperty)` - A data property entity.
/// * `AnnotationProperty(IRI)` - An annotation property entity.
/// * `NamedIndividual(IRI)` - A named individual entity.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Entity {
    Class(Class),
    Datatype(Datatype),
    ObjectProperty(ObjectProperty),
    DataProperty(DataProperty),
    AnnotationProperty(IRI),
    NamedIndividual(IRI),
}

/// Represents an individual in the ontology.
///
/// Individuals are the basic objects in an OWL 2 ontology. They can be either named
/// (identified by an IRI) or anonymous (identified by a NodeID).
///
/// # Variants
///
/// * `Named(IRI)` - A named individual identified by an IRI.
/// * `Anonymous(NodeID)` - An anonymous individual identified by a NodeID.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::{Individual, IRI, NodeID};
///
/// let named_individual = Individual::Named(IRI("http://example.com/john".to_string()));
/// let anonymous_individual = Individual::Anonymous(NodeID("_:b1".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Individual {
    Named(IRI),
    Anonymous(NodeID),
}

/// Represents a literal value, which can have a datatype or a language tag.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Literal {
    pub value: String,
    pub datatype: Datatype,
    pub lang: Option<String>,
}

/// A ClassExpression is a class or a boolean combination of classes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// An ObjectPropertyExpression is an object property or an inverse of an object property.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectPropertyExpression {
    ObjectProperty(ObjectProperty),
    InverseObjectProperty(ObjectProperty),
    ObjectPropertyChain(Vec<ObjectPropertyExpression>),
}

/// Axioms about classes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClassAxiom {
    SubClassOf {
        sub_class: ClassExpression,
        super_class: ClassExpression,
    },
    EquivalentClasses {
        classes: Vec<ClassExpression>,
    },
    DisjointClasses {
        classes: Vec<ClassExpression>,
    },
    DisjointUnion {
        class: Class,
        disjoint_classes: Vec<ClassExpression>,
    },
}

/// Axioms about object properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectPropertyAxiom {
    SubObjectPropertyOf {
        sub_property: ObjectPropertyExpression,
        super_property: ObjectPropertyExpression,
    },
    EquivalentObjectProperties {
        properties: Vec<ObjectPropertyExpression>,
    },
    DisjointObjectProperties {
        properties: Vec<ObjectPropertyExpression>,
    },
    InverseObjectProperties {
        prop1: ObjectPropertyExpression,
        prop2: ObjectPropertyExpression,
    },
    ObjectPropertyDomain {
        property: ObjectPropertyExpression,
        domain: ClassExpression,
    },
    ObjectPropertyRange {
        property: ObjectPropertyExpression,
        range: ClassExpression,
    },
    FunctionalObjectProperty { property: ObjectPropertyExpression },
    InverseFunctionalObjectProperty { property: ObjectPropertyExpression },
    ReflexiveObjectProperty { property: ObjectPropertyExpression },
    IrreflexiveObjectProperty { property: ObjectPropertyExpression },
    SymmetricObjectProperty { property: ObjectPropertyExpression },
    AsymmetricObjectProperty { property: ObjectPropertyExpression },
    TransitiveObjectProperty { property: ObjectPropertyExpression },
}

/// Represents a data range in OWL 2.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataRange {
    Datatype(Datatype),
    DataIntersectionOf(Vec<DataRange>),
    DataUnionOf(Vec<DataRange>),
    DataComplementOf(Box<DataRange>),
    DataOneOf(Vec<Literal>),
    DatatypeRestriction {
        datatype: Datatype,
        restrictions: Vec<(IRI, Literal)>,
    },
}

/// Axioms about data properties.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataPropertyAxiom {
    SubDataPropertyOf {
        sub_property: DataProperty,
        super_property: DataProperty,
    },
    EquivalentDataProperties {
        properties: Vec<DataProperty>,
    },
    DisjointDataProperties {
        properties: Vec<DataProperty>,
    },
    DataPropertyDomain {
        property: DataProperty,
        domain: ClassExpression,
    },
    DataPropertyRange {
        property: DataProperty,
        range: DataRange,
    },
    FunctionalDataProperty { property: DataProperty },
}

/// Assertions about individuals.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// A general axiom type that encompasses all specific axiom types.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Axiom {
    Class(ClassAxiom),
    ObjectProperty(ObjectPropertyAxiom),
    DataProperty(DataPropertyAxiom),
    Assertion(Assertion),
}

/// Represents a complete OWL 2 ontology.
///
/// An ontology consists of a set of axioms that describe the relationships
/// between classes, properties, and individuals. It may also import other ontologies.
///
/// # Fields
///
/// * `direct_imports` - IRIs of ontologies that are directly imported by this ontology.
/// * `axioms` - The axioms that make up this ontology.
///
/// # Examples
///
/// ```rust
/// use owl2_rs::Ontology;
///
/// let ontology = Ontology::default();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Ontology {
    pub direct_imports: Vec<IRI>,
    pub axioms: Vec<Axiom>,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iri_creation() {
        let iri = IRI("http://example.com/class".to_string());
        assert_eq!(iri.0, "http://example.com/class");
    }

    #[test]
    fn test_entity_creation() {
        let class_entity = Entity::Class(Class(IRI("http://example.com/class".to_string())));
        assert!(matches!(class_entity, Entity::Class(_)));

        let datatype_entity = Entity::Datatype(Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string())));
        assert!(matches!(datatype_entity, Entity::Datatype(_)));

        let object_property_entity = Entity::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string())));
        assert!(matches!(object_property_entity, Entity::ObjectProperty(_)));

        let data_property_entity = Entity::DataProperty(DataProperty(IRI("http://example.com/hasAge".to_string())));
        assert!(matches!(data_property_entity, Entity::DataProperty(_)));

        let annotation_property_entity = Entity::AnnotationProperty(IRI("http://example.com/comment".to_string()));
        assert!(matches!(annotation_property_entity, Entity::AnnotationProperty(_)));

        let named_individual_entity = Entity::NamedIndividual(IRI("http://example.com/individual1".to_string()));
        assert!(matches!(named_individual_entity, Entity::NamedIndividual(_)));
    }

    #[test]
    fn test_class_creation() {
        let class = Class(IRI("http://example.com/MyClass".to_string()));
        assert_eq!(class.0 .0, "http://example.com/MyClass");
    }

    #[test]
    fn test_datatype_creation() {
        let datatype = Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string()));
        assert_eq!(datatype.0 .0, "http://www.w3.org/2001/XMLSchema#string");
    }

    #[test]
    fn test_object_property_creation() {
        let object_property = ObjectProperty(IRI("http://example.com/hasPart".to_string()));
        assert_eq!(object_property.0 .0, "http://example.com/hasPart");
    }

    #[test]
    fn test_data_property_creation() {
        let data_property = DataProperty(IRI("http://example.com/hasAge".to_string()));
        assert_eq!(data_property.0 .0, "http://example.com/hasAge");
    }

    #[test]
    fn test_individual_creation() {
        let named_individual = Individual::Named(IRI("http://example.com/individual".to_string()));
        assert!(matches!(named_individual, Individual::Named(_)));

        let anonymous_individual = Individual::Anonymous(NodeID("_:b1".to_string()));
        assert!(matches!(anonymous_individual, Individual::Anonymous(_)));
    }

    #[test]
    fn test_node_id_creation() {
        let node_id = NodeID("_:b1".to_string());
        assert_eq!(node_id.0, "_:b1");
    }

    #[test]
    fn test_literal_creation() {
        let literal = Literal {
            value: "hello".to_string(),
            datatype: Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string())),
            lang: Some("en".to_string()),
        };
        assert_eq!(literal.value, "hello");
        assert_eq!(literal.lang, Some("en".to_string()));
    }

    #[test]
    fn test_subclassof_axiom() {
        let child_class = Class(IRI("http://example.com/child".to_string()));
        let parent_class = Class(IRI("http://example.com/parent".to_string()));

        let axiom = ClassAxiom::SubClassOf {
            sub_class: ClassExpression::Class(child_class.clone()),
            super_class: ClassExpression::Class(parent_class.clone()),
        };

        if let ClassAxiom::SubClassOf { sub_class, super_class } = axiom {
            assert_eq!(sub_class, ClassExpression::Class(child_class));
            assert_eq!(super_class, ClassExpression::Class(parent_class));
        } else {
            panic!("Axiom is not SubClassOf");
        }
    }

    #[test]
    fn test_subobjectpropertyof_axiom() {
        let sub_prop = ObjectProperty(IRI("http://example.com/subProp".to_string()));
        let super_prop = ObjectProperty(IRI("http://example.com/superProp".to_string()));

        let axiom = ObjectPropertyAxiom::SubObjectPropertyOf {
            sub_property: ObjectPropertyExpression::ObjectProperty(sub_prop.clone()),
            super_property: ObjectPropertyExpression::ObjectProperty(super_prop.clone()),
        };

        if let ObjectPropertyAxiom::SubObjectPropertyOf { sub_property, super_property } = axiom {
            assert_eq!(sub_property, ObjectPropertyExpression::ObjectProperty(sub_prop));
            assert_eq!(super_property, ObjectPropertyExpression::ObjectProperty(super_prop));
        } else {
            panic!("Axiom is not SubObjectPropertyOf");
        }
    }

    #[test]
    fn test_subdatapropertyof_axiom() {
        let sub_prop = DataProperty(IRI("http://example.com/subProp".to_string()));
        let super_prop = DataProperty(IRI("http://example.com/superProp".to_string()));

        let axiom = DataPropertyAxiom::SubDataPropertyOf {
            sub_property: sub_prop.clone(),
            super_property: super_prop.clone(),
        };

        if let DataPropertyAxiom::SubDataPropertyOf { sub_property, super_property } = axiom {
            assert_eq!(sub_property, sub_prop);
            assert_eq!(super_property, super_prop);
        } else {
            panic!("Axiom is not SubDataPropertyOf");
        }
    }

    #[test]
    fn test_classassertion_axiom() {
        let class = Class(IRI("http://example.com/class".to_string()));
        let individual = Individual::Named(IRI("http://example.com/individual".to_string()));

        let axiom = Assertion::ClassAssertion {
            class: ClassExpression::Class(class.clone()),
            individual: individual.clone(),
        };

        if let Assertion::ClassAssertion { class: c, individual: i } = axiom {
            assert_eq!(c, ClassExpression::Class(class));
            assert_eq!(i, individual);
        } else {
            panic!("Axiom is not ClassAssertion");
        }
    }

    #[test]
    fn test_complex_class_expressions() {
        let class1 = Class(IRI("http://example.com/class1".to_string()));
        let class2 = Class(IRI("http://example.com/class2".to_string()));
        
        // Test ObjectIntersectionOf
        let intersection = ClassExpression::ObjectIntersectionOf(vec![
            ClassExpression::Class(class1.clone()),
            ClassExpression::Class(class2.clone()),
        ]);
        assert_eq!(intersection, ClassExpression::ObjectIntersectionOf(vec![
            ClassExpression::Class(class1.clone()),
            ClassExpression::Class(class2.clone()),
        ]));
        
        // Test ObjectUnionOf
        let union = ClassExpression::ObjectUnionOf(vec![
            ClassExpression::Class(class1.clone()),
            ClassExpression::Class(class2.clone()),
        ]);
        assert_eq!(union, ClassExpression::ObjectUnionOf(vec![
            ClassExpression::Class(class1.clone()),
            ClassExpression::Class(class2.clone()),
        ]));
        
        // Test ObjectComplementOf
        let complement = ClassExpression::ObjectComplementOf(Box::new(ClassExpression::Class(class1.clone())));
        assert_eq!(complement, ClassExpression::ObjectComplementOf(Box::new(ClassExpression::Class(class1.clone()))));
    }

    #[test]
    fn test_data_range() {
        let datatype = Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string()));
        let data_range = DataRange::Datatype(datatype.clone());
        assert_eq!(data_range, DataRange::Datatype(datatype));
    }

    #[test]
    fn test_ontology_creation() {
        let mut ontology = Ontology::default();
        let class = Class(IRI("http://example.com/class".to_string()));
        let individual = Individual::Named(IRI("http://example.com/individual".to_string()));

        let axiom = Axiom::Assertion(Assertion::ClassAssertion {
            class: ClassExpression::Class(class.clone()),
            individual: individual.clone(),
        });

        ontology.axioms.push(axiom);

        assert_eq!(ontology.axioms.len(), 1);
    }

    #[test]
    fn test_parser_iri() {
        use crate::parser::OWLParser;

        let input = "<http://example.com/iri>";
        let iri = OWLParser::parse_iri(input).unwrap();
        assert_eq!(iri, IRI("http://example.com/iri".to_string()));
    }

    #[test]
    fn test_parser_prefix() {
        use crate::parser::{OWLParser, Prefix};

        let input = "Prefix(ex:=<http://example.com/>)";
        let prefix = OWLParser::parse_prefix(input).unwrap();
        assert_eq!(prefix, Prefix { name: "ex".to_string(), iri: IRI("http://example.com/".to_string()) });
    }

    #[test]
    fn test_parser_entity() {
        use crate::parser::OWLParser;

        let input_class = "Class(<http://example.com/MyClass>)";
        let entity_class = OWLParser::parse_entity(input_class).unwrap();
        assert_eq!(entity_class, Entity::Class(Class(IRI("http://example.com/MyClass".to_string()))));

        let input_datatype = "Datatype(<http://www.w3.org/2001/XMLSchema#string>)";
        let entity_datatype = OWLParser::parse_entity(input_datatype).unwrap();
        assert_eq!(entity_datatype, Entity::Datatype(Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string()))));

        let input_object_property = "ObjectProperty(<http://example.com/hasPart>)";
        let entity_object_property = OWLParser::parse_entity(input_object_property).unwrap();
        assert_eq!(entity_object_property, Entity::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))));

        let input_data_property = "DataProperty(<http://example.com/hasValue>)";
        let entity_data_property = OWLParser::parse_entity(input_data_property).unwrap();
        assert_eq!(entity_data_property, Entity::DataProperty(DataProperty(IRI("http://example.com/hasValue".to_string()))));

        let input_annotation_property = "AnnotationProperty(<http://example.com/comment>)";
        let entity_annotation_property = OWLParser::parse_entity(input_annotation_property).unwrap();
        assert_eq!(entity_annotation_property, Entity::AnnotationProperty(IRI("http://example.com/comment".to_string())));

        let input_named_individual = "NamedIndividual(<http://example.com/individual1>)";
        let entity_named_individual = OWLParser::parse_entity(input_named_individual).unwrap();
        assert_eq!(entity_named_individual, Entity::NamedIndividual(IRI("http://example.com/individual1".to_string())));
    }

    #[test]
    fn test_parser_literal() {
        use crate::parser::OWLParser;

        let input_simple = r#""hello""#;
        let literal_simple = OWLParser::parse_literal(input_simple).unwrap();
        assert_eq!(literal_simple, Literal { value: "hello".to_string(), datatype: Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string())), lang: None });

        let input_lang = r#""hello"@en"#;
        let literal_lang = OWLParser::parse_literal(input_lang).unwrap();
        assert_eq!(literal_lang, Literal { value: "hello".to_string(), datatype: Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string())), lang: Some("en".to_string()) });
    }

    #[test]
    fn test_parser_class_expression() {
        use crate::parser::OWLParser;

        let input_class = "Class(<http://example.com/MyClass>)";
        let class_expression = OWLParser::parse_class_expression(input_class).unwrap();
        assert_eq!(class_expression, ClassExpression::Class(Class(IRI("http://example.com/MyClass".to_string()))));
    }

    #[test]
    fn test_parser_object_property_expression() {
        use crate::parser::OWLParser;

        let input_op = "ObjectProperty(<http://example.com/hasPart>)";
        let op_expr = OWLParser::parse_object_property_expression(input_op).unwrap();
        assert_eq!(op_expr, ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))));

        let input_inv_op = "ObjectInverseOf(ObjectProperty(<http://example.com/hasPart>))";
        let inv_op_expr = OWLParser::parse_object_property_expression(input_inv_op).unwrap();
        assert_eq!(inv_op_expr, ObjectPropertyExpression::InverseObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))));
    }

    #[test]
    fn test_parser_class_axiom() {
        use crate::parser::OWLParser;

        let input_subclass = "SubClassOf(Class(<http://example.com/Child>) Class(<http://example.com/Parent>))";
        let axiom_subclass = OWLParser::parse_class_axiom(input_subclass).unwrap();
        assert_eq!(axiom_subclass, ClassAxiom::SubClassOf {
            sub_class: ClassExpression::Class(Class(IRI("http://example.com/Child".to_string()))),
            super_class: ClassExpression::Class(Class(IRI("http://example.com/Parent".to_string()))),
        });

        let input_equivalent = "EquivalentClasses(Class(<http://example.com/Class1>) Class(<http://example.com/Class2>))";
        let axiom_equivalent = OWLParser::parse_class_axiom(input_equivalent).unwrap();
        assert_eq!(axiom_equivalent, ClassAxiom::EquivalentClasses {
            classes: vec![
                ClassExpression::Class(Class(IRI("http://example.com/Class1".to_string()))),
                ClassExpression::Class(Class(IRI("http://example.com/Class2".to_string()))),
            ],
        });

        let input_disjoint = "DisjointClasses(Class(<http://example.com/ClassA>) Class(<http://example.com/ClassB>))";
        let axiom_disjoint = OWLParser::parse_class_axiom(input_disjoint).unwrap();
        assert_eq!(axiom_disjoint, ClassAxiom::DisjointClasses {
            classes: vec![
                ClassExpression::Class(Class(IRI("http://example.com/ClassA".to_string()))),
                ClassExpression::Class(Class(IRI("http://example.com/ClassB".to_string()))),
            ],
        });

        let input_disjoint_union = "DisjointUnion(Class(<http://example.com/UnionClass>) Class(<http://example.com/Part1>) Class(<http://example.com/Part2>))";
        let axiom_disjoint_union = OWLParser::parse_class_axiom(input_disjoint_union).unwrap();
        assert_eq!(axiom_disjoint_union, ClassAxiom::DisjointUnion {
            class: Class(IRI("http://example.com/UnionClass".to_string())),
            disjoint_classes: vec![
                ClassExpression::Class(Class(IRI("http://example.com/Part1".to_string()))),
                ClassExpression::Class(Class(IRI("http://example.com/Part2".to_string()))),
            ],
        });
    }

    #[test]
    fn test_parser_ontology() {
        use crate::parser::OWLParser;

        let input = "Ontology(<http://example.com/ontology> SubClassOf(Class(<http://example.com/Child>) Class(<http://example.com/Parent>)))";
        let ontology = OWLParser::parse_ontology(input).unwrap();
        assert_eq!(ontology.axioms.len(), 1);
    }

    #[test]
    fn test_parser_data_property() {
        use crate::parser::OWLParser;

        let input = "DataProperty(<http://example.com/hasAge>)";
        let entity = OWLParser::parse_entity(input).unwrap();
        assert!(matches!(entity, Entity::DataProperty(_)));
    }

    #[test]
    fn test_parser_complex_ontology() {
        use crate::parser::OWLParser;

        // Let's build up the ontology with more axioms
        let input = r#"Ontology(<http://example.com/ontology>
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
        let ontology = OWLParser::parse_ontology(input).unwrap();
        assert_eq!(ontology.axioms.len(), 9);
    }

    #[test]
    fn test_parser_object_property_axiom() {
        use crate::parser::OWLParser;

        let input_sub_op = "SubObjectPropertyOf(ObjectProperty(<http://example.com/subProp>) ObjectProperty(<http://example.com/superProp>))";
        let axiom_sub_op = OWLParser::parse_object_property_axiom(input_sub_op).unwrap();
        assert_eq!(axiom_sub_op, ObjectPropertyAxiom::SubObjectPropertyOf {
            sub_property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/subProp".to_string()))),
            super_property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/superProp".to_string()))),
        });

        let input_equivalent_op = "EquivalentObjectProperties(ObjectProperty(<http://example.com/Prop1>) ObjectProperty(<http://example.com/Prop2>))";
        let axiom_equivalent_op = OWLParser::parse_object_property_axiom(input_equivalent_op).unwrap();
        assert_eq!(axiom_equivalent_op, ObjectPropertyAxiom::EquivalentObjectProperties {
            properties: vec![
                ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/Prop1".to_string()))),
                ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/Prop2".to_string()))),
            ],
        });

        let input_disjoint_op = "DisjointObjectProperties(ObjectProperty(<http://example.com/PropA>) ObjectProperty(<http://example.com/PropB>))";
        let axiom_disjoint_op = OWLParser::parse_object_property_axiom(input_disjoint_op).unwrap();
        assert_eq!(axiom_disjoint_op, ObjectPropertyAxiom::DisjointObjectProperties {
            properties: vec![
                ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/PropA".to_string()))),
                ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/PropB".to_string()))),
            ],
        });

        let input_inverse_op = "InverseObjectProperties(ObjectProperty(<http://example.com/Prop1>) ObjectProperty(<http://example.com/Prop2>))";
        let axiom_inverse_op = OWLParser::parse_object_property_axiom(input_inverse_op).unwrap();
        assert_eq!(axiom_inverse_op, ObjectPropertyAxiom::InverseObjectProperties {
            prop1: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/Prop1".to_string()))),
            prop2: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/Prop2".to_string()))),
        });

        let input_domain_op = "ObjectPropertyDomain(ObjectProperty(<http://example.com/hasPart>) Class(<http://example.com/DomainClass>))";
        let axiom_domain_op = OWLParser::parse_object_property_axiom(input_domain_op).unwrap();
        assert_eq!(axiom_domain_op, ObjectPropertyAxiom::ObjectPropertyDomain {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
            domain: ClassExpression::Class(Class(IRI("http://example.com/DomainClass".to_string()))),
        });

        let input_range_op = "ObjectPropertyRange(ObjectProperty(<http://example.com/hasPart>) Class(<http://example.com/RangeClass>))";
        let axiom_range_op = OWLParser::parse_object_property_axiom(input_range_op).unwrap();
        assert_eq!(axiom_range_op, ObjectPropertyAxiom::ObjectPropertyRange {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
            range: ClassExpression::Class(Class(IRI("http://example.com/RangeClass".to_string()))),
        });

        let input_functional_op = "FunctionalObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_functional_op = OWLParser::parse_object_property_axiom(input_functional_op).unwrap();
        assert_eq!(axiom_functional_op, ObjectPropertyAxiom::FunctionalObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });

        let input_inverse_functional_op = "InverseFunctionalObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_inverse_functional_op = OWLParser::parse_object_property_axiom(input_inverse_functional_op).unwrap();
        assert_eq!(axiom_inverse_functional_op, ObjectPropertyAxiom::InverseFunctionalObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });

        let input_reflexive_op = "ReflexiveObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_reflexive_op = OWLParser::parse_object_property_axiom(input_reflexive_op).unwrap();
        assert_eq!(axiom_reflexive_op, ObjectPropertyAxiom::ReflexiveObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });

        let input_irreflexive_op = "IrreflexiveObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_irreflexive_op = OWLParser::parse_object_property_axiom(input_irreflexive_op).unwrap();
        assert_eq!(axiom_irreflexive_op, ObjectPropertyAxiom::IrreflexiveObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });

        let input_symmetric_op = "SymmetricObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_symmetric_op = OWLParser::parse_object_property_axiom(input_symmetric_op).unwrap();
        assert_eq!(axiom_symmetric_op, ObjectPropertyAxiom::SymmetricObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });

        let input_asymmetric_op = "AsymmetricObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_asymmetric_op = OWLParser::parse_object_property_axiom(input_asymmetric_op).unwrap();
        assert_eq!(axiom_asymmetric_op, ObjectPropertyAxiom::AsymmetricObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });

        let input_transitive_op = "TransitiveObjectProperty(ObjectProperty(<http://example.com/hasPart>))";
        let axiom_transitive_op = OWLParser::parse_object_property_axiom(input_transitive_op).unwrap();
        assert_eq!(axiom_transitive_op, ObjectPropertyAxiom::TransitiveObjectProperty {
            property: ObjectPropertyExpression::ObjectProperty(ObjectProperty(IRI("http://example.com/hasPart".to_string()))),
        });
    }
}