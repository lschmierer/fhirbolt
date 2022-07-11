// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct CompartmentDefinitionResource {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#param: Vec<super::super::types::String>,
    pub r#documentation: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct CompartmentDefinition {
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#status: super::super::types::Code,
    pub r#name: super::super::types::String,
    pub r#version: Option<super::super::types::String>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#resource: Vec<CompartmentDefinitionResource>,
    pub r#code: super::super::types::Code,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#url: super::super::types::Uri,
    pub r#search: super::super::types::Boolean,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#purpose: Option<super::super::types::Markdown>,
}
