// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct AdverseEventSuspectEntityCausality {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#assessment: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_relatedness: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct AdverseEventSuspectEntity {
    pub r#causality: Vec<AdverseEventSuspectEntityCausality>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#instance: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct AdverseEvent {
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#detected: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#event: Option<Box<super::super::types::CodeableConcept>>,
    pub r#resulting_condition: Vec<Box<super::super::types::Reference>>,
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#actuality: super::super::types::Code,
    pub r#seriousness: Option<Box<super::super::types::CodeableConcept>>,
    pub r#study: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reference_document: Vec<Box<super::super::types::Reference>>,
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#suspect_entity: Vec<AdverseEventSuspectEntity>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#subject_medical_history: Vec<Box<super::super::types::Reference>>,
}
