// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ObservationValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum ObservationEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
    Instant(Box<super::super::types::Instant>),
}
#[derive(Debug, Clone)]
pub enum ObservationComponentValue {
    Quantity(Box<super::super::types::Quantity>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Range(Box<super::super::types::Range>),
    Ratio(Box<super::super::types::Ratio>),
    SampledData(Box<super::super::types::SampledData>),
    Time(Box<super::super::types::Time>),
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct ObservationComponent {
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reference_range: Vec<ObservationReferenceRange>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#value: Option<ObservationComponentValue>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ObservationReferenceRange {
    pub r#id: Option<std::string::String>,
    pub r#low: Option<Box<super::super::types::Quantity>>,
    pub r#age: Option<Box<super::super::types::Range>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#high: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<super::super::types::String>,
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Observation {
    pub r#has_member: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#value: Option<ObservationValue>,
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#effective: Option<ObservationEffective>,
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status: super::super::types::Code,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#issued: Option<super::super::types::Instant>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#component: Vec<ObservationComponent>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#reference_range: Vec<ObservationReferenceRange>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
}
