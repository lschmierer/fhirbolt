// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MetadataResource {
    pub r#publisher: Option<super::super::types::String>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#version: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#language: Option<super::super::types::Code>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
}
