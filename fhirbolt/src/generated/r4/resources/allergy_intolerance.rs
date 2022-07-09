// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
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
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#onset: Option<super::super::types::DateTime>,
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    pub r#exposure_route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#severity: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#manifestation: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct AllergyIntolerance {
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#criticality: Option<super::super::types::Code>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#last_occurrence: Option<super::super::types::DateTime>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#category: Vec<super::super::types::Code>,
    pub r#onset: Option<AllergyIntoleranceOnset>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#reaction: Vec<AllergyIntoleranceReaction>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<super::super::types::Code>,
}
