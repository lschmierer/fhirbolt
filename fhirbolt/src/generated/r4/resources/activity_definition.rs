// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ActivityDefinitionProduct {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct ActivityDefinitionParticipant {
    pub r#type: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ActivityDefinitionDynamicValue {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: super::super::types::String,
    pub r#expression: Box<super::super::types::Expression>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum ActivityDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    Duration(Box<super::super::types::Duration>),
}
#[derive(Debug, Clone)]
pub enum ActivityDefinitionSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ActivityDefinition {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#product: Option<ActivityDefinitionProduct>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#transform: Option<super::super::types::Canonical>,
    pub r#reviewer: Vec<Box<super::super::types::ContactDetail>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#subtitle: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#usage: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#approval_date: Option<super::super::types::Date>,
    pub r#library: Vec<super::super::types::Canonical>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#related_artifact: Vec<Box<super::super::types::RelatedArtifact>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#last_review_date: Option<super::super::types::Date>,
    pub r#kind: Option<super::super::types::Code>,
    pub r#intent: Option<super::super::types::Code>,
    pub r#endorser: Vec<Box<super::super::types::ContactDetail>>,
    pub r#participant: Vec<ActivityDefinitionParticipant>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#status: super::super::types::Code,
    pub r#observation_result_requirement: Vec<Box<super::super::types::Reference>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#dynamic_value: Vec<ActivityDefinitionDynamicValue>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#specimen_requirement: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#effective_period: Option<Box<super::super::types::Period>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#author: Vec<Box<super::super::types::ContactDetail>>,
    pub r#title: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#topic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#editor: Vec<Box<super::super::types::ContactDetail>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#timing: Option<ActivityDefinitionTiming>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#observation_requirement: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Option<ActivityDefinitionSubject>,
}
