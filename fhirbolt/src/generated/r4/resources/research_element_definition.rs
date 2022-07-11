// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionCharacteristicStudyEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionCharacteristicParticipantEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub enum ResearchElementDefinitionCharacteristicDefinition {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Canonical(Box<super::super::types::Canonical>),
    Expression(Box<super::super::types::Expression>),
    DataRequirement(Box<super::super::types::DataRequirement>),
}
#[derive(Debug, Clone)]
pub struct ResearchElementDefinitionCharacteristic {
    pub r#study_effective_description: Option<super::super::types::String>,
    pub r#participant_effective_group_measure: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#study_effective_group_measure: Option<super::super::types::Code>,
    pub r#participant_effective_description: Option<super::super::types::String>,
    pub r#study_effective: Option<ResearchElementDefinitionCharacteristicStudyEffective>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#exclude: Option<super::super::types::Boolean>,
    pub r#participant_effective:
        Option<ResearchElementDefinitionCharacteristicParticipantEffective>,
    pub r#unit_of_measure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#study_effective_time_from_start: Option<Box<super::super::types::Duration>>,
    pub r#definition: ResearchElementDefinitionCharacteristicDefinition,
    pub r#participant_effective_time_from_start: Option<Box<super::super::types::Duration>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#usage_context: Vec<Box<super::super::types::UsageContext>>,
}
#[derive(Debug, Clone)]
pub struct ResearchElementDefinition {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#version: Option<super::super::types::String>,
    pub r#short_title: Option<super::super::types::String>,
    pub r#comment: Vec<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#usage: Option<super::super::types::String>,
    pub r#variable_type: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#title: Option<super::super::types::String>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Option<ResearchElementDefinitionSubject>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#name: Option<super::super::types::String>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#id: Option<std::string::String>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#characteristic: Vec<ResearchElementDefinitionCharacteristic>,
    pub r#type: super::super::types::Code,
    pub r#publisher: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
