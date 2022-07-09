// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MetadataResource {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#title: Option<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#status: super::super::types::Code,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
}
