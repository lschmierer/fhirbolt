// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
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
pub struct ObservationComponent {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#value: Option<ObservationComponentValue>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reference_range: Vec<ObservationReferenceRange>,
}
#[derive(Debug, Clone)]
pub struct ObservationReferenceRange {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<super::super::types::String>,
    pub r#low: Option<Box<super::super::types::Quantity>>,
    pub r#age: Option<Box<super::super::types::Range>>,
    pub r#high: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Observation {
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    pub r#has_member: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#component: Vec<ObservationComponent>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#issued: Option<super::super::types::Instant>,
    pub r#data_absent_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#specimen: Option<Box<super::super::types::Reference>>,
    pub r#interpretation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#value: Option<ObservationValue>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#reference_range: Vec<ObservationReferenceRange>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#effective: Option<ObservationEffective>,
}
