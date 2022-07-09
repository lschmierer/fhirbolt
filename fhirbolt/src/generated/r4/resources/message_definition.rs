// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MessageDefinitionFocus {
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#max: Option<super::super::types::String>,
    pub r#min: super::super::types::UnsignedInt,
    pub r#code: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum MessageDefinitionEvent {
    Coding(Box<super::super::types::Coding>),
    Uri(Box<super::super::types::Uri>),
}
#[derive(Debug, Clone)]
pub struct MessageDefinitionAllowedResponse {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#situation: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#message: super::super::types::Canonical,
}
#[derive(Debug, Clone)]
pub struct MessageDefinition {
    pub r#version: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#graph: Vec<super::super::types::Canonical>,
    pub r#replaces: Vec<super::super::types::Canonical>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#category: Option<super::super::types::Code>,
    pub r#name: Option<super::super::types::String>,
    pub r#focus: Vec<MessageDefinitionFocus>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#event: MessageDefinitionEvent,
    pub r#title: Option<super::super::types::String>,
    pub r#allowed_response: Vec<MessageDefinitionAllowedResponse>,
    pub r#id: Option<std::string::String>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: super::super::types::Code,
    pub r#date: super::super::types::DateTime,
    pub r#url: Option<super::super::types::Uri>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#base: Option<super::super::types::Canonical>,
    pub r#parent: Vec<super::super::types::Canonical>,
    pub r#response_required: Option<super::super::types::Code>,
    pub r#purpose: Option<super::super::types::Markdown>,
}
