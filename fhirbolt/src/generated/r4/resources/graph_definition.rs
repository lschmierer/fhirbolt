// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct GraphDefinitionLinkTargetCompartment {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#use: super::super::types::Code,
    pub r#code: super::super::types::Code,
    pub r#expression: Option<super::super::types::String>,
    pub r#rule: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct GraphDefinitionLinkTarget {
    pub r#compartment: Vec<GraphDefinitionLinkTargetCompartment>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#params: Option<super::super::types::String>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#link: Vec<GraphDefinitionLink>,
}
#[derive(Debug, Clone)]
pub struct GraphDefinitionLink {
    pub r#target: Vec<GraphDefinitionLinkTarget>,
    pub r#slice_name: Option<super::super::types::String>,
    pub r#max: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#min: Option<super::super::types::Integer>,
}
#[derive(Debug, Clone)]
pub struct GraphDefinition {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#version: Option<super::super::types::String>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#link: Vec<GraphDefinitionLink>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#start: super::super::types::Code,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#name: super::super::types::String,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
}
