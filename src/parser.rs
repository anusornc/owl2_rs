use crate::{Axiom, Class, ClassAxiom, ClassExpression, DataProperty, DataPropertyAxiom, DataRange, Datatype, Entity, IRI, Individual, Literal, ObjectProperty, ObjectPropertyAxiom, ObjectPropertyExpression, Assertion};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct OWLParser;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Prefix {
    pub name: String,
    pub iri: IRI,
}

impl OWLParser {
    pub fn parse_iri(input: &str) -> Result<IRI, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::iri, input)?;
        let pair = pairs.next().unwrap();
        let inner = pair.into_inner().find(|p| p.as_rule() == Rule::iri_content).unwrap();
        Ok(IRI(inner.as_str().to_string()))
    }

    pub fn parse_prefix(input: &str) -> Result<Prefix, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::prefix, input)?;
        let pair = pairs.next().unwrap();
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let iri_str = inner.next().unwrap().into_inner().next().unwrap().as_str();
        let iri = IRI(iri_str.to_string());
        Ok(Prefix { name, iri })
    }

    pub fn parse_entity(input: &str) -> Result<Entity, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::entity, input)?;
        let entity_rule_pair = pairs.next().unwrap(); // This is the pair for the matched entity rule (e.g., class, datatype)

        let inner_rule_pair = entity_rule_pair.into_inner().next().unwrap(); // Get the inner rule (class, datatype, etc.)

        let entity = match inner_rule_pair.as_rule() {
            Rule::class => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::Class(Class(OWLParser::parse_iri(iri_str).unwrap()))
            },
            Rule::datatype => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::Datatype(Datatype(OWLParser::parse_iri(iri_str).unwrap()))
            },
            Rule::object_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::ObjectProperty(ObjectProperty(OWLParser::parse_iri(iri_str).unwrap()))
            },
            Rule::data_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::DataProperty(DataProperty(OWLParser::parse_iri(iri_str).unwrap()))
            },
            Rule::annotation_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::AnnotationProperty(OWLParser::parse_iri(iri_str).unwrap())
            },
            Rule::named_individual => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::NamedIndividual(OWLParser::parse_iri(iri_str).unwrap())
            },
            _ => unreachable!(),
        };
        Ok(entity)
    }

    pub fn parse_literal(input: &str) -> Result<Literal, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::literal, input)?;
        let literal_pair = pairs.next().unwrap();
        let mut inner_pairs = literal_pair.into_inner();

        let value = inner_pairs.next().unwrap().as_str().to_string();
        let mut datatype = Datatype(IRI("http://www.w3.org/2001/XMLSchema#string".to_string())); // Default to string for now
        let mut lang: Option<String> = None;

        if let Some(next_pair) = inner_pairs.next() {
            match next_pair.as_rule() {
                Rule::iri => {
                    // This is the datatype IRI
                    datatype = Datatype(OWLParser::parse_iri(next_pair.as_str()).unwrap());
                }
                Rule::lang_tag => {
                    lang = Some(next_pair.as_str().to_string());
                }
                _ => unreachable!(),
            }
        }

        Ok(Literal { value, datatype, lang })
    }

    pub fn parse_class_expression(input: &str) -> Result<ClassExpression, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::class_expression, input)?;
        let class_expression_pair = pairs.next().unwrap();
        let inner_rule_pair = class_expression_pair.into_inner().next().unwrap();

        let class_expression = match inner_rule_pair.as_rule() {
            Rule::class => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                ClassExpression::Class(Class(OWLParser::parse_iri(iri_str).unwrap()))
            },
            Rule::object_intersection_of => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str()).unwrap()).collect();
                ClassExpression::ObjectIntersectionOf(classes)
            },
            Rule::object_union_of => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str()).unwrap()).collect();
                ClassExpression::ObjectUnionOf(classes)
            },
            Rule::object_complement_of => {
                let class_expr = OWLParser::parse_class_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ClassExpression::ObjectComplementOf(Box::new(class_expr))
            },
            Rule::object_one_of => {
                let individuals: Vec<Individual> = inner_rule_pair.into_inner().map(|p| {
                    let entity = OWLParser::parse_entity(p.as_str()).unwrap();
                    if let Entity::NamedIndividual(iri) = entity {
                        Individual::Named(iri)
                    } else {
                        panic!("Expected a NamedIndividual in ObjectOneOf, but got {:?}", entity);
                    }
                }).collect();
                ClassExpression::ObjectOneOf(individuals)
            },
            Rule::object_some_values_from => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let filler = Box::new(OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap());
                ClassExpression::ObjectSomeValuesFrom { property, filler }
            },
            Rule::object_all_values_from => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let filler = Box::new(OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap());
                ClassExpression::ObjectAllValuesFrom { property, filler }
            },
            Rule::object_has_value => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let value_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let value = if let Entity::NamedIndividual(iri) = value_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ObjectHasValue, but got {:?}", value_entity);
                };
                ClassExpression::ObjectHasValue { property, value }
            },
            Rule::object_has_self => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ClassExpression::ObjectHasSelf(property)
            },
            Rule::object_min_cardinality => {
                let mut inner = inner_rule_pair.into_inner();
                let min: u32 = inner.next().unwrap().as_str().parse().unwrap();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let filler = if let Some(filler_pair) = inner.next() {
                    Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str()).unwrap()))
                } else {
                    None
                };
                ClassExpression::ObjectMinCardinality { min, property, filler }
            },
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
            Rule::object_exact_cardinality => {
                let mut inner = inner_rule_pair.into_inner();
                let cardinality: u32 = inner.next().unwrap().as_str().parse().unwrap();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let filler = if let Some(filler_pair) = inner.next() {
                    Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str()).unwrap()))
                } else {
                    None
                };
                ClassExpression::ObjectExactCardinality { cardinality, property, filler }
            },
            _ => unreachable!(),
        };
        Ok(class_expression)
    }

    pub fn parse_object_property(input: &str) -> Result<ObjectProperty, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::object_property, input)?;
        let object_property_pair = pairs.next().unwrap();
        let iri_str = object_property_pair.into_inner().next().unwrap().as_str();
        Ok(ObjectProperty(OWLParser::parse_iri(iri_str).unwrap()))
    }

    pub fn parse_object_property_expression(input: &str) -> Result<ObjectPropertyExpression, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::object_property_expression, input)?;
        let object_property_expression_pair = pairs.next().unwrap();

        let inner_rule_pair = object_property_expression_pair.into_inner().next().unwrap(); // Get the inner rule (object_property or object_inverse_of_rule)

        let object_property_expression = match inner_rule_pair.as_rule() {
            Rule::object_property => {
                let object_property = OWLParser::parse_object_property(inner_rule_pair.as_str()).unwrap();
                ObjectPropertyExpression::ObjectProperty(object_property)
            },
            Rule::object_inverse_of_rule => {
                let object_property = OWLParser::parse_object_property(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyExpression::InverseObjectProperty(object_property)
            },
            _ => unreachable!(),
        };
        Ok(object_property_expression)
    }

    pub fn parse_class_axiom(input: &str) -> Result<ClassAxiom, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::class_axiom, input)?;
        let class_axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = class_axiom_pair.into_inner().next().unwrap();

        let class_axiom = match inner_rule_pair.as_rule() {
            Rule::sub_class_of => {
                let mut inner = inner_rule_pair.into_inner();
                let sub_class = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                let super_class = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                ClassAxiom::SubClassOf { sub_class, super_class }
            },
            Rule::equivalent_classes => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str()).unwrap()).collect();
                ClassAxiom::EquivalentClasses { classes }
            },
            Rule::disjoint_classes => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str()).unwrap()).collect();
                ClassAxiom::DisjointClasses { classes }
            },
            Rule::disjoint_union => {
                let mut inner = inner_rule_pair.into_inner();
                let class_expr = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                let class = if let ClassExpression::Class(c) = class_expr {
                    c
                } else {
                    panic!("Expected a Class in DisjointUnion, but got {:?}", class_expr);
                };
                let disjoint_classes: Vec<ClassExpression> = inner.map(|p| OWLParser::parse_class_expression(p.as_str()).unwrap()).collect();
                ClassAxiom::DisjointUnion { class, disjoint_classes }
            },
            _ => unreachable!(),
        };
        Ok(class_axiom)
    }

    pub fn parse_object_property_axiom(input: &str) -> Result<ObjectPropertyAxiom, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::object_property_axiom, input)?;
        let object_property_axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = object_property_axiom_pair.into_inner().next().unwrap();

        let object_property_axiom = match inner_rule_pair.as_rule() {
            Rule::sub_object_property_of => {
                let mut inner = inner_rule_pair.into_inner();
                let sub_property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let super_property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::SubObjectPropertyOf { sub_property, super_property }
            },
            Rule::equivalent_object_properties => {
                let properties: Vec<ObjectPropertyExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_object_property_expression(p.as_str()).unwrap()).collect();
                ObjectPropertyAxiom::EquivalentObjectProperties { properties }
            },
            Rule::disjoint_object_properties => {
                let properties: Vec<ObjectPropertyExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_object_property_expression(p.as_str()).unwrap()).collect();
                ObjectPropertyAxiom::DisjointObjectProperties { properties }
            },
            Rule::inverse_object_properties => {
                let mut inner = inner_rule_pair.into_inner();
                let prop1 = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let prop2 = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::InverseObjectProperties { prop1, prop2 }
            },
            Rule::object_property_domain => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let domain = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::ObjectPropertyDomain { property, domain }
            },
            Rule::object_property_range => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let range = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::ObjectPropertyRange { property, range }
            },
            Rule::functional_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::FunctionalObjectProperty { property }
            },
            Rule::inverse_functional_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::InverseFunctionalObjectProperty { property }
            },
            Rule::reflexive_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::ReflexiveObjectProperty { property }
            },
            Rule::irreflexive_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::IrreflexiveObjectProperty { property }
            },
            Rule::symmetric_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::SymmetricObjectProperty { property }
            },
            Rule::asymmetric_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::AsymmetricObjectProperty { property }
            },
            Rule::transitive_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                ObjectPropertyAxiom::TransitiveObjectProperty { property }
            },
            _ => unreachable!(),
        };
        Ok(object_property_axiom)
    }

    pub fn parse_data_property_axiom(input: &str) -> Result<DataPropertyAxiom, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::data_property_axiom, input)?;
        let data_property_axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = data_property_axiom_pair.into_inner().next().unwrap();

        let data_property_axiom = match inner_rule_pair.as_rule() {
            Rule::sub_data_property_of => {
                let mut inner = inner_rule_pair.into_inner();
                let sub_property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let sub_property = if let Entity::DataProperty(dp) = sub_property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in SubDataPropertyOf, but got {:?}", sub_property_entity);
                };
                let super_property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let super_property = if let Entity::DataProperty(dp) = super_property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in SubDataPropertyOf, but got {:?}", super_property_entity);
                };
                DataPropertyAxiom::SubDataPropertyOf { sub_property, super_property }
            },
            Rule::equivalent_data_properties => {
                let properties: Vec<DataProperty> = inner_rule_pair.into_inner().map(|p| {
                    let entity = OWLParser::parse_entity(p.as_str()).unwrap();
                    if let Entity::DataProperty(dp) = entity {
                        dp
                    } else {
                        panic!("Expected a DataProperty in EquivalentDataProperties, but got {:?}", entity);
                    }
                }).collect();
                DataPropertyAxiom::EquivalentDataProperties { properties }
            },
            Rule::disjoint_data_properties => {
                let properties: Vec<DataProperty> = inner_rule_pair.into_inner().map(|p| {
                    let entity = OWLParser::parse_entity(p.as_str()).unwrap();
                    if let Entity::DataProperty(dp) = entity {
                        dp
                    } else {
                        panic!("Expected a DataProperty in DisjointDataProperties, but got {:?}", entity);
                    }
                }).collect();
                DataPropertyAxiom::DisjointDataProperties { properties }
            },
            Rule::data_property_domain => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in DataPropertyDomain, but got {:?}", property_entity);
                };
                let domain = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                DataPropertyAxiom::DataPropertyDomain { property, domain }
            },
            Rule::data_property_range => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in DataPropertyRange, but got {:?}", property_entity);
                };
                let range_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let range = if let Entity::Datatype(dt) = range_entity {
                    DataRange::Datatype(dt)
                } else {
                    panic!("Expected a Datatype in DataPropertyRange, but got {:?}", range_entity);
                };
                DataPropertyAxiom::DataPropertyRange { property, range }
            },
            Rule::functional_data_property => {
                let property_entity = OWLParser::parse_entity(inner_rule_pair.into_inner().next().unwrap().as_str()).unwrap();
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in FunctionalDataProperty, but got {:?}", property_entity);
                };
                DataPropertyAxiom::FunctionalDataProperty { property }
            },
            _ => unreachable!(),
        };
        Ok(data_property_axiom)
    }

    pub fn parse_assertion(input: &str) -> Result<Assertion, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::assertion, input)?;
        let assertion_pair = pairs.next().unwrap();
        let inner_rule_pair = assertion_pair.into_inner().next().unwrap();

        let assertion = match inner_rule_pair.as_rule() {
            Rule::same_individual => {
                let individuals: Vec<Individual> = inner_rule_pair.into_inner().map(|p| {
                    let entity = OWLParser::parse_entity(p.as_str()).unwrap();
                    if let Entity::NamedIndividual(iri) = entity {
                        Individual::Named(iri)
                    } else {
                        panic!("Expected a NamedIndividual in SameIndividual, but got {:?}", entity);
                    }
                }).collect();
                Assertion::SameIndividual { individuals }
            },
            Rule::different_individuals => {
                let individuals: Vec<Individual> = inner_rule_pair.into_inner().map(|p| {
                    let entity = OWLParser::parse_entity(p.as_str()).unwrap();
                    if let Entity::NamedIndividual(iri) = entity {
                        Individual::Named(iri)
                    } else {
                        panic!("Expected a NamedIndividual in DifferentIndividuals, but got {:?}", entity);
                    }
                }).collect();
                Assertion::DifferentIndividuals { individuals }
            },
            Rule::class_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let class_expression = OWLParser::parse_class_expression(inner.next().unwrap().as_str()).unwrap();
                let individual_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let individual = if let Entity::NamedIndividual(iri) = individual_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ClassAssertion, but got {:?}", individual_entity);
                };
                Assertion::ClassAssertion { class: class_expression, individual }
            },
            Rule::object_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ObjectPropertyAssertion, but got {:?}", source_entity);
                };
                let target_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let target = if let Entity::NamedIndividual(iri) = target_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ObjectPropertyAssertion, but got {:?}", target_entity);
                };
                Assertion::ObjectPropertyAssertion { property, source, target }
            },
            Rule::data_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in DataPropertyAssertion, but got {:?}", property_entity);
                };
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in DataPropertyAssertion, but got {:?}", source_entity);
                };
                let target = OWLParser::parse_literal(inner.next().unwrap().as_str()).unwrap();
                Assertion::DataPropertyAssertion { property, source, target }
            },
            Rule::negative_object_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str()).unwrap();
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in NegativeObjectPropertyAssertion, but got {:?}", source_entity);
                };
                let target_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let target = if let Entity::NamedIndividual(iri) = target_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in NegativeObjectPropertyAssertion, but got {:?}", target_entity);
                };
                Assertion::NegativeObjectPropertyAssertion { property, source, target }
            },
            Rule::negative_data_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in NegativeDataPropertyAssertion, but got {:?}", property_entity);
                };
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str()).unwrap();
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in NegativeDataPropertyAssertion, but got {:?}", source_entity);
                };
                let target = OWLParser::parse_literal(inner.next().unwrap().as_str()).unwrap();
                Assertion::NegativeDataPropertyAssertion { property, source, target }
            },
            _ => unreachable!(),
        };
        Ok(assertion)
    }

    pub fn parse_axiom(input: &str) -> Result<Axiom, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::axiom, input)?;
        let axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = axiom_pair.into_inner().next().unwrap();

        let axiom = match inner_rule_pair.as_rule() {
            Rule::class_axiom => Axiom::Class(OWLParser::parse_class_axiom(inner_rule_pair.as_str()).unwrap()),
            Rule::object_property_axiom => Axiom::ObjectProperty(OWLParser::parse_object_property_axiom(inner_rule_pair.as_str()).unwrap()),
            Rule::data_property_axiom => Axiom::DataProperty(OWLParser::parse_data_property_axiom(inner_rule_pair.as_str()).unwrap()),
            Rule::assertion => Axiom::Assertion(OWLParser::parse_assertion(inner_rule_pair.as_str()).unwrap()),
            _ => unreachable!(),
        };
        Ok(axiom)
    }

    pub fn parse_ontology(input: &str) -> Result<crate::Ontology, pest::error::Error<Rule>> {
        let mut pairs = OWLParser::parse(Rule::ontology, input)?;
        let ontology_pair = pairs.next().unwrap();
        let mut inner = ontology_pair.into_inner();

        // The first optional element is the ontology IRI
        let mut ontology = crate::Ontology::default();
        
        // Check if the first element is an IRI
        if let Some(first_pair) = inner.peek() {
            if first_pair.as_rule() == Rule::iri {
                let iri_pair = inner.next().unwrap();
                let _iri = OWLParser::parse_iri(iri_pair.as_str()).unwrap();
                // For now, we'll just note that we have an IRI but we're not storing it
                // In a more complete implementation, we would store the ontology IRI
            }
        }

        // Parse all the axioms
        for axiom_pair in inner {
            if axiom_pair.as_rule() == Rule::axiom {
                let axiom = OWLParser::parse_axiom(axiom_pair.as_str()).unwrap();
                ontology.axioms.push(axiom);
            }
        }

        Ok(ontology)
    }
}