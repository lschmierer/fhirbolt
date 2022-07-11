// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum AllergyIntoleranceOnset {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct AllergyIntoleranceReaction {
    pub r#id: Option<std::string::String>,
    pub r#severity: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#manifestation: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#onset: Option<super::super::types::DateTime>,
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#exposure_route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
#[derive(Debug, Clone)]
pub struct AllergyIntolerance {
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#onset: Option<AllergyIntoleranceOnset>,
    pub r#type: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#criticality: Option<super::super::types::Code>,
    pub r#language: Option<super::super::types::Code>,
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#reaction: Vec<AllergyIntoleranceReaction>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#last_occurrence: Option<super::super::types::DateTime>,
    pub r#category: Vec<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
}
