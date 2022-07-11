// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum EvidenceVariableCharacteristicParticipantEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub enum EvidenceVariableCharacteristicDefinition {
    Reference(Box<super::super::types::Reference>),
    Canonical(Box<super::super::types::Canonical>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Expression(Box<super::super::types::Expression>),
    DataRequirement(Box<super::super::types::DataRequirement>),
    TriggerDefinition(Box<super::super::types::TriggerDefinition>),
}
#[derive(Debug, Clone)]
pub struct EvidenceVariableCharacteristic {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#usage_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#id: Option<std::string::String>,
    pub r#exclude: Option<super::super::types::Boolean>,
    pub r#time_from_start: Option<Box<super::super::types::Duration>>,
    pub r#participant_effective: Option<EvidenceVariableCharacteristicParticipantEffective>,
    pub r#group_measure: Option<super::super::types::Code>,
    pub r#definition: EvidenceVariableCharacteristicDefinition,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct EvidenceVariable {
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#version: Option<super::super::types::String>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#characteristic: Vec<EvidenceVariableCharacteristic>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#short_title: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#title: Option<super::super::types::String>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
