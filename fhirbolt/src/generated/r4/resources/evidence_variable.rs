// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
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
    pub r#exclude: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#time_from_start: Option<Box<super::super::types::Duration>>,
    pub r#participant_effective: Option<EvidenceVariableCharacteristicParticipantEffective>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#group_measure: Option<super::super::types::Code>,
    pub r#usage_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#definition: EvidenceVariableCharacteristicDefinition,
}
#[derive(Debug, Clone)]
pub struct EvidenceVariable {
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#characteristic: Vec<EvidenceVariableCharacteristic>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#name: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#short_title: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
}
