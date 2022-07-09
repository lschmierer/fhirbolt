// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CompartmentDefinitionResource {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#documentation: Option<super::super::types::String>,
    pub r#code: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#param: Vec<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct CompartmentDefinition {
    pub r#date: Option<super::super::types::DateTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#resource: Vec<CompartmentDefinitionResource>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#url: super::super::types::Uri,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
    pub r#version: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#code: super::super::types::Code,
    pub r#search: super::super::types::Boolean,
}
