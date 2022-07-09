// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct AdverseEventSuspectEntityCausality {
    pub r#assessment: Option<Box<super::super::types::CodeableConcept>>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_relatedness: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct AdverseEventSuspectEntity {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#instance: Box<super::super::types::Reference>,
    pub r#causality: Vec<AdverseEventSuspectEntityCausality>,
}
#[derive(Debug, Clone)]
pub struct AdverseEvent {
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#reference_document: Vec<Box<super::super::types::Reference>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#event: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detected: Option<super::super::types::DateTime>,
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    pub r#study: Vec<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#resulting_condition: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#subject_medical_history: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#actuality: super::super::types::Code,
    pub r#seriousness: Option<Box<super::super::types::CodeableConcept>>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#suspect_entity: Vec<AdverseEventSuspectEntity>,
}
