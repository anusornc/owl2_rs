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
    pub fn parse_iri(input: &str) -> Result<IRI, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::iri, input)?;
        let pair = pairs.next().unwrap();
        let inner = pair.into_inner().find(|p| p.as_rule() == Rule::iri_content).unwrap();
        Ok(IRI(inner.as_str().to_string()))
    }

    pub fn parse_prefix(input: &str) -> Result<Prefix, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::prefix, input)?;
        let pair = pairs.next().unwrap();
        let mut inner = pair.into_inner();
        let name = inner.next().unwrap().as_str().to_string();
        let iri_str = inner.next().unwrap().into_inner().next().unwrap().as_str();
        let iri = IRI(iri_str.to_string());
        Ok(Prefix { name, iri })
    }

    pub fn parse_entity(input: &str) -> Result<Entity, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::entity, input)?;
        let entity_rule_pair = pairs.next().unwrap(); // This is the pair for the matched entity rule (e.g., class, datatype)

        let inner_rule_pair = entity_rule_pair.into_inner().next().unwrap(); // Get the inner rule (class, datatype, etc.)

        let entity = match inner_rule_pair.as_rule() {
            Rule::class => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::Class(Class(OWLParser::parse_iri(iri_str)?))
            },
            Rule::datatype => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::Datatype(Datatype(OWLParser::parse_iri(iri_str)?))
            },
            Rule::object_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::ObjectProperty(ObjectProperty(OWLParser::parse_iri(iri_str)?))
            },
            Rule::data_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::DataProperty(DataProperty(OWLParser::parse_iri(iri_str)?))
            },
            Rule::annotation_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::AnnotationProperty(OWLParser::parse_iri(iri_str)?)
            },
            Rule::named_individual => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                Entity::NamedIndividual(OWLParser::parse_iri(iri_str)?)
            },
            _ => unreachable!(),
        };
        Ok(entity)
    }

    pub fn parse_literal(input: &str) -> Result<Literal, Box<pest::error::Error<Rule>>> {
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
                    datatype = Datatype(OWLParser::parse_iri(next_pair.as_str())?);
                }
                Rule::lang_tag => {
                    lang = Some(next_pair.as_str().to_string());
                }
                _ => unreachable!(),
            }
        }

        Ok(Literal { value, datatype, lang })
    }

    pub fn parse_class_expression(input: &str) -> Result<ClassExpression, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::class_expression, input)?;
        let class_expression_pair = pairs.next().unwrap();
        let inner_rule_pair = class_expression_pair.into_inner().next().unwrap();

        let class_expression = match inner_rule_pair.as_rule() {
            Rule::class => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                ClassExpression::Class(Class(OWLParser::parse_iri(iri_str)?))
            },
            Rule::object_intersection_of => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ClassExpression::ObjectIntersectionOf(classes)
            },
            Rule::object_union_of => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ClassExpression::ObjectUnionOf(classes)
            },
            Rule::object_complement_of => {
                let class_expr = OWLParser::parse_class_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ClassExpression::ObjectComplementOf(Box::new(class_expr))
            },
            Rule::object_one_of => {
                let mut individuals = Vec::new();
                for p in inner_rule_pair.into_inner() {
                    let entity = OWLParser::parse_entity(p.as_str())?;
                    if let Entity::NamedIndividual(iri) = entity {
                        individuals.push(Individual::Named(iri));
                    } else {
                        panic!("Expected a NamedIndividual in ObjectOneOf, but got {:?}", entity);
                    }
                }
                ClassExpression::ObjectOneOf(individuals)
            },
            Rule::object_some_values_from => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let filler = Box::new(OWLParser::parse_class_expression(inner.next().unwrap().as_str())?);
                ClassExpression::ObjectSomeValuesFrom { property, filler }
            },
            Rule::object_all_values_from => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let filler = Box::new(OWLParser::parse_class_expression(inner.next().unwrap().as_str())?);
                ClassExpression::ObjectAllValuesFrom { property, filler }
            },
            Rule::object_has_value => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let value_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let value = if let Entity::NamedIndividual(iri) = value_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ObjectHasValue, but got {:?}", value_entity);
                };
                ClassExpression::ObjectHasValue { property, value }
            },
            Rule::object_has_self => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ClassExpression::ObjectHasSelf(property)
            },
            Rule::object_min_cardinality => {
                let span = inner_rule_pair.as_span();
                let text = inner_rule_pair.as_str();
                // Extract the numeric value from the text
                // Format: ObjectMinCardinality(NUMBER object_property_expression class_expression?)
                let start = text.find('(').unwrap() + 1;
                let end = text.find(' ').unwrap();
                let min_str = &text[start..end];
                let min: u32 = min_str.parse().map_err(|e| {
                    Box::new(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: format!("Failed to parse cardinality '{}': {}", min_str, e),
                        },
                        span
                    ))
                })?;
                
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let filler = if let Some(filler_pair) = inner.next() {
                    Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str())?))
                } else {
                    None
                };
                ClassExpression::ObjectMinCardinality { min, property, filler }
            },
            Rule::object_max_cardinality => {
                let span = inner_rule_pair.as_span();
                let text = inner_rule_pair.as_str();
                // Extract the numeric value from the text
                // Format: ObjectMaxCardinality(NUMBER object_property_expression class_expression?)
                let start = text.find('(').unwrap() + 1;
                let end = text.find(' ').unwrap();
                let max_str = &text[start..end];
                let max: u32 = max_str.parse().map_err(|e| {
                    Box::new(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: format!("Failed to parse cardinality '{}': {}", max_str, e),
                        },
                        span
                    ))
                })?;
                
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let filler = if let Some(filler_pair) = inner.next() {
                    Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str())?))
                } else {
                    None
                };
                ClassExpression::ObjectMaxCardinality { max, property, filler }
            },
            Rule::object_exact_cardinality => {
                let span = inner_rule_pair.as_span();
                let text = inner_rule_pair.as_str();
                // Extract the numeric value from the text
                // Format: ObjectExactCardinality(NUMBER object_property_expression class_expression?)
                let start = text.find('(').unwrap() + 1;
                let end = text.find(' ').unwrap();
                let cardinality_str = &text[start..end];
                let cardinality: u32 = cardinality_str.parse().map_err(|e| {
                    Box::new(pest::error::Error::new_from_span(
                        pest::error::ErrorVariant::CustomError {
                            message: format!("Failed to parse cardinality '{}': {}", cardinality_str, e),
                        },
                        span
                    ))
                })?;
                
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let filler = if let Some(filler_pair) = inner.next() {
                    Some(Box::new(OWLParser::parse_class_expression(filler_pair.as_str())?))
                } else {
                    None
                };
                ClassExpression::ObjectExactCardinality { cardinality, property, filler }
            },
            _ => unreachable!(),
        };
        Ok(class_expression)
    }

    pub fn parse_object_property(input: &str) -> Result<ObjectProperty, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::object_property, input)?;
        let object_property_pair = pairs.next().unwrap();
        let iri_str = object_property_pair.into_inner().next().unwrap().as_str();
        Ok(ObjectProperty(OWLParser::parse_iri(iri_str)?))
    }

    pub fn parse_object_property_expression(input: &str) -> Result<ObjectPropertyExpression, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::object_property_expression, input)?;
        let object_property_expression_pair = pairs.next().unwrap();
        let inner_rule_pair = object_property_expression_pair.into_inner().next().unwrap();

        let object_property_expression = match inner_rule_pair.as_rule() {
            Rule::object_property => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().as_str();
                ObjectPropertyExpression::ObjectProperty(ObjectProperty(OWLParser::parse_iri(iri_str)?))
            },
            Rule::object_inverse_of_rule => {
                let iri_str = inner_rule_pair.into_inner().next().unwrap().into_inner().next().unwrap().as_str();
                ObjectPropertyExpression::InverseObjectProperty(ObjectProperty(OWLParser::parse_iri(iri_str)?))
            },
            Rule::object_property_chain => {
                let properties: Vec<ObjectPropertyExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_object_property_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ObjectPropertyExpression::ObjectPropertyChain(properties)
            },
            _ => unreachable!(),
        };
        Ok(object_property_expression)
    }

    pub fn parse_class_axiom(input: &str) -> Result<ClassAxiom, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::class_axiom, input)?;
        let class_axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = class_axiom_pair.into_inner().next().unwrap();

        let class_axiom = match inner_rule_pair.as_rule() {
            Rule::sub_class_of => {
                let mut inner = inner_rule_pair.into_inner();
                let sub_class = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                let super_class = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                ClassAxiom::SubClassOf { sub_class, super_class }
            },
            Rule::equivalent_classes => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ClassAxiom::EquivalentClasses { classes }
            },
            Rule::disjoint_classes => {
                let classes: Vec<ClassExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_class_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ClassAxiom::DisjointClasses { classes }
            },
            Rule::disjoint_union => {
                let mut inner = inner_rule_pair.into_inner();
                let class_expr = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                let class = if let ClassExpression::Class(c) = class_expr {
                    c
                } else {
                    panic!("Expected a Class in DisjointUnion, but got {:?}", class_expr);
                };
                let disjoint_classes: Vec<ClassExpression> = inner.map(|p| OWLParser::parse_class_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ClassAxiom::DisjointUnion { class, disjoint_classes }
            },
            _ => unreachable!(),
        };
        Ok(class_axiom)
    }

    pub fn parse_object_property_axiom(input: &str) -> Result<ObjectPropertyAxiom, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::object_property_axiom, input)?;
        let object_property_axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = object_property_axiom_pair.into_inner().next().unwrap();

        let object_property_axiom = match inner_rule_pair.as_rule() {
            Rule::sub_object_property_of => {
                let mut inner = inner_rule_pair.into_inner();
                let sub_property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let super_property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                ObjectPropertyAxiom::SubObjectPropertyOf { sub_property, super_property }
            },
            Rule::equivalent_object_properties => {
                let properties: Vec<ObjectPropertyExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_object_property_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ObjectPropertyAxiom::EquivalentObjectProperties { properties }
            },
            Rule::disjoint_object_properties => {
                let properties: Vec<ObjectPropertyExpression> = inner_rule_pair.into_inner().map(|p| OWLParser::parse_object_property_expression(p.as_str())).collect::<Result<Vec<_>, _>>()?;
                ObjectPropertyAxiom::DisjointObjectProperties { properties }
            },
            Rule::inverse_object_properties => {
                let mut inner = inner_rule_pair.into_inner();
                let prop1 = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let prop2 = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                ObjectPropertyAxiom::InverseObjectProperties { prop1, prop2 }
            },
            Rule::object_property_domain => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let domain = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                ObjectPropertyAxiom::ObjectPropertyDomain { property, domain }
            },
            Rule::object_property_range => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let range = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                ObjectPropertyAxiom::ObjectPropertyRange { property, range }
            },
            Rule::functional_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::FunctionalObjectProperty { property }
            },
            Rule::inverse_functional_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::InverseFunctionalObjectProperty { property }
            },
            Rule::reflexive_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::ReflexiveObjectProperty { property }
            },
            Rule::irreflexive_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::IrreflexiveObjectProperty { property }
            },
            Rule::symmetric_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::SymmetricObjectProperty { property }
            },
            Rule::asymmetric_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::AsymmetricObjectProperty { property }
            },
            Rule::transitive_object_property => {
                let property = OWLParser::parse_object_property_expression(inner_rule_pair.into_inner().next().unwrap().as_str())?;
                ObjectPropertyAxiom::TransitiveObjectProperty { property }
            },
            _ => unreachable!(),
        };
        Ok(object_property_axiom)
    }

    pub fn parse_data_property_axiom(input: &str) -> Result<DataPropertyAxiom, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::data_property_axiom, input)?;
        let data_property_axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = data_property_axiom_pair.into_inner().next().unwrap();

        let data_property_axiom = match inner_rule_pair.as_rule() {
            Rule::sub_data_property_of => {
                let mut inner = inner_rule_pair.into_inner();
                let sub_property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let sub_property = if let Entity::DataProperty(dp) = sub_property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in SubDataPropertyOf, but got {:?}", sub_property_entity);
                };
                let super_property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let super_property = if let Entity::DataProperty(dp) = super_property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in SubDataPropertyOf, but got {:?}", super_property_entity);
                };
                DataPropertyAxiom::SubDataPropertyOf { sub_property, super_property }
            },
            Rule::equivalent_data_properties => {
                let mut properties = Vec::new();
                for p in inner_rule_pair.into_inner() {
                    let entity = OWLParser::parse_entity(p.as_str())?;
                    if let Entity::DataProperty(dp) = entity {
                        properties.push(dp);
                    } else {
                        panic!("Expected a DataProperty in EquivalentDataProperties, but got {:?}", entity);
                    }
                }
                DataPropertyAxiom::EquivalentDataProperties { properties }
            },
            Rule::disjoint_data_properties => {
                let mut properties = Vec::new();
                for p in inner_rule_pair.into_inner() {
                    let entity = OWLParser::parse_entity(p.as_str())?;
                    if let Entity::DataProperty(dp) = entity {
                        properties.push(dp);
                    } else {
                        panic!("Expected a DataProperty in DisjointDataProperties, but got {:?}", entity);
                    }
                }
                DataPropertyAxiom::DisjointDataProperties { properties }
            },
            Rule::data_property_domain => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in DataPropertyDomain, but got {:?}", property_entity);
                };
                let domain = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                DataPropertyAxiom::DataPropertyDomain { property, domain }
            },
            Rule::data_property_range => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in DataPropertyRange, but got {:?}", property_entity);
                };
                let range_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let range = if let Entity::Datatype(dt) = range_entity {
                    DataRange::Datatype(dt)
                } else {
                    panic!("Expected a Datatype in DataPropertyRange, but got {:?}", range_entity);
                };
                DataPropertyAxiom::DataPropertyRange { property, range }
            },
            Rule::functional_data_property => {
                let property_entity = OWLParser::parse_entity(inner_rule_pair.into_inner().next().unwrap().as_str())?;
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

    pub fn parse_assertion(input: &str) -> Result<Assertion, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::assertion, input)?;
        let assertion_pair = pairs.next().unwrap();
        let inner_rule_pair = assertion_pair.into_inner().next().unwrap();

        let assertion = match inner_rule_pair.as_rule() {
            Rule::same_individual => {
                let mut individuals = Vec::new();
                for p in inner_rule_pair.into_inner() {
                    let entity = OWLParser::parse_entity(p.as_str())?;
                    if let Entity::NamedIndividual(iri) = entity {
                        individuals.push(Individual::Named(iri));
                    } else {
                        panic!("Expected a NamedIndividual in SameIndividual, but got {:?}", entity);
                    }
                }
                Assertion::SameIndividual { individuals }
            },
            Rule::different_individuals => {
                let mut individuals = Vec::new();
                for p in inner_rule_pair.into_inner() {
                    let entity = OWLParser::parse_entity(p.as_str())?;
                    if let Entity::NamedIndividual(iri) = entity {
                        individuals.push(Individual::Named(iri));
                    } else {
                        panic!("Expected a NamedIndividual in DifferentIndividuals, but got {:?}", entity);
                    }
                }
                Assertion::DifferentIndividuals { individuals }
            },
            Rule::class_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let class_expression = OWLParser::parse_class_expression(inner.next().unwrap().as_str())?;
                let individual_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let individual = if let Entity::NamedIndividual(iri) = individual_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ClassAssertion, but got {:?}", individual_entity);
                };
                Assertion::ClassAssertion { class: class_expression, individual }
            },
            Rule::object_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ObjectPropertyAssertion, but got {:?}", source_entity);
                };
                let target_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let target = if let Entity::NamedIndividual(iri) = target_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in ObjectPropertyAssertion, but got {:?}", target_entity);
                };
                Assertion::ObjectPropertyAssertion { property, source, target }
            },
            Rule::data_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in DataPropertyAssertion, but got {:?}", property_entity);
                };
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in DataPropertyAssertion, but got {:?}", source_entity);
                };
                let target = OWLParser::parse_literal(inner.next().unwrap().as_str())?;
                Assertion::DataPropertyAssertion { property, source, target }
            },
            Rule::negative_object_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property = OWLParser::parse_object_property_expression(inner.next().unwrap().as_str())?;
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in NegativeObjectPropertyAssertion, but got {:?}", source_entity);
                };
                let target_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let target = if let Entity::NamedIndividual(iri) = target_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in NegativeObjectPropertyAssertion, but got {:?}", target_entity);
                };
                Assertion::NegativeObjectPropertyAssertion { property, source, target }
            },
            Rule::negative_data_property_assertion => {
                let mut inner = inner_rule_pair.into_inner();
                let property_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let property = if let Entity::DataProperty(dp) = property_entity {
                    dp
                } else {
                    panic!("Expected a DataProperty in NegativeDataPropertyAssertion, but got {:?}", property_entity);
                };
                let source_entity = OWLParser::parse_entity(inner.next().unwrap().as_str())?;
                let source = if let Entity::NamedIndividual(iri) = source_entity {
                    Individual::Named(iri)
                } else {
                    panic!("Expected a NamedIndividual in NegativeDataPropertyAssertion, but got {:?}", source_entity);
                };
                let target = OWLParser::parse_literal(inner.next().unwrap().as_str())?;
                Assertion::NegativeDataPropertyAssertion { property, source, target }
            },
            _ => unreachable!(),
        };
        Ok(assertion)
    }

    pub fn parse_axiom(input: &str) -> Result<Axiom, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::axiom, input)?;
        let axiom_pair = pairs.next().unwrap();
        let inner_rule_pair = axiom_pair.into_inner().next().unwrap();

        let axiom = match inner_rule_pair.as_rule() {
            Rule::class_axiom => Axiom::Class(OWLParser::parse_class_axiom(inner_rule_pair.as_str())?),
            Rule::object_property_axiom => Axiom::ObjectProperty(OWLParser::parse_object_property_axiom(inner_rule_pair.as_str())?),
            Rule::data_property_axiom => Axiom::DataProperty(OWLParser::parse_data_property_axiom(inner_rule_pair.as_str())?),
            Rule::assertion => Axiom::Assertion(OWLParser::parse_assertion(inner_rule_pair.as_str())?),
            _ => unreachable!(),
        };
        Ok(axiom)
    }

    pub fn parse_ontology(input: &str) -> Result<crate::Ontology, Box<pest::error::Error<Rule>>> {
        let mut pairs = OWLParser::parse(Rule::ontology, input)?;
        let ontology_pair = pairs.next().unwrap();
        let mut inner = ontology_pair.into_inner();

        // The first optional element is the ontology IRI
        let mut ontology = crate::Ontology::default();
        
        // Check if the first element is an IRI
        if let Some(first_pair) = inner.peek() {
            if first_pair.as_rule() == Rule::iri {
                let iri_pair = inner.next().unwrap();
                let _iri = OWLParser::parse_iri(iri_pair.as_str())?;
                // For now, we'll just note that we have an IRI but we're not storing it
                // In a more complete implementation, we would store the ontology IRI
            }
        }

        // Parse all the axioms
        for axiom_pair in inner {
            if axiom_pair.as_rule() == Rule::axiom {
                let axiom = OWLParser::parse_axiom(axiom_pair.as_str())?;
                ontology.axioms.push(axiom);
            }
            // Skip comments (they don't need to be processed)
        }

        Ok(ontology)
    }
}