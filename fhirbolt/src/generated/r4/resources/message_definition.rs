// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MessageDefinitionEvent {
    Coding(Box<super::super::types::Coding>),
    Uri(Box<super::super::types::Uri>),
}
#[derive(Debug, Clone)]
pub struct MessageDefinitionFocus {
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#code: super::super::types::Code,
    pub r#max: Option<super::super::types::String>,
    pub r#min: super::super::types::UnsignedInt,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MessageDefinitionAllowedResponse {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#situation: Option<super::super::types::Markdown>,
    pub r#message: super::super::types::Canonical,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MessageDefinition {
    pub r#language: Option<super::super::types::Code>,
    pub r#date: super::super::types::DateTime,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#replaces: Vec<super::super::types::Canonical>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#title: Option<super::super::types::String>,
    pub r#event: MessageDefinitionEvent,
    pub r#focus: Vec<MessageDefinitionFocus>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#allowed_response: Vec<MessageDefinitionAllowedResponse>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#parent: Vec<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#base: Option<super::super::types::Canonical>,
    pub r#category: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#response_required: Option<super::super::types::Code>,
    pub r#graph: Vec<super::super::types::Canonical>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#copyright: Option<super::super::types::Markdown>,
}
