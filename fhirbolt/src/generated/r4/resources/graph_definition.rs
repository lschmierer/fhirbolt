// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct GraphDefinitionLinkTargetCompartment {
    pub r#code: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#use: super::super::types::Code,
    pub r#rule: super::super::types::Code,
    pub r#description: Option<super::super::types::String>,
    pub r#expression: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct GraphDefinitionLinkTarget {
    pub r#link: Vec<GraphDefinitionLink>,
    pub r#compartment: Vec<GraphDefinitionLinkTargetCompartment>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#params: Option<super::super::types::String>,
    pub r#profile: Option<super::super::types::Canonical>,
}
#[derive(Debug, Clone)]
pub struct GraphDefinitionLink {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#path: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#slice_name: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#max: Option<super::super::types::String>,
    pub r#min: Option<super::super::types::Integer>,
    pub r#target: Vec<GraphDefinitionLinkTarget>,
}
#[derive(Debug, Clone)]
pub struct GraphDefinition {
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#status: super::super::types::Code,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#start: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
    pub r#version: Option<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#link: Vec<GraphDefinitionLink>,
}
