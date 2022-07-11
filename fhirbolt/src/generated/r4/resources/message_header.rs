// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MessageHeaderEvent {
    Coding(Box<super::super::types::Coding>),
    Uri(Box<super::super::types::Uri>),
}
#[derive(Debug, Clone)]
pub struct MessageHeaderResponse {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: super::super::types::Id,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#details: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct MessageHeaderSource {
    pub r#name: Option<super::super::types::String>,
    pub r#endpoint: super::super::types::Url,
    pub r#software: Option<super::super::types::String>,
    pub r#contact: Option<Box<super::super::types::ContactPoint>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MessageHeaderDestination {
    pub r#receiver: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#endpoint: super::super::types::Url,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#target: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct MessageHeader {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#response: Option<MessageHeaderResponse>,
    pub r#id: Option<std::string::String>,
    pub r#sender: Option<Box<super::super::types::Reference>>,
    pub r#responsible: Option<Box<super::super::types::Reference>>,
    pub r#focus: Vec<Box<super::super::types::Reference>>,
    pub r#source: MessageHeaderSource,
    pub r#destination: Vec<MessageHeaderDestination>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#event: MessageHeaderEvent,
    pub r#definition: Option<super::super::types::Canonical>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
