// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CodeSystemProperty {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#uri: Option<super::super::types::Uri>,
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub enum CodeSystemConceptPropertyValue {
    Code(Box<super::super::types::Code>),
    Coding(Box<super::super::types::Coding>),
    String(Box<super::super::types::String>),
    Integer(Box<super::super::types::Integer>),
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
    Decimal(Box<super::super::types::Decimal>),
}
#[derive(Debug, Clone)]
pub struct CodeSystemConceptProperty {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: super::super::types::Code,
    pub r#value: CodeSystemConceptPropertyValue,
}
#[derive(Debug, Clone)]
pub struct CodeSystemConceptDesignation {
    pub r#language: Option<super::super::types::Code>,
    pub r#value: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#use: Option<Box<super::super::types::Coding>>,
}
#[derive(Debug, Clone)]
pub struct CodeSystemConcept {
    pub r#definition: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#display: Option<super::super::types::String>,
    pub r#property: Vec<CodeSystemConceptProperty>,
    pub r#code: super::super::types::Code,
    pub r#concept: Vec<CodeSystemConcept>,
    pub r#designation: Vec<CodeSystemConceptDesignation>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CodeSystemFilter {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#operator: Vec<super::super::types::Code>,
    pub r#value: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct CodeSystem {
    pub r#date: Option<super::super::types::DateTime>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#title: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#name: Option<super::super::types::String>,
    pub r#compositional: Option<super::super::types::Boolean>,
    pub r#property: Vec<CodeSystemProperty>,
    pub r#concept: Vec<CodeSystemConcept>,
    pub r#value_set: Option<super::super::types::Canonical>,
    pub r#case_sensitive: Option<super::super::types::Boolean>,
    pub r#filter: Vec<CodeSystemFilter>,
    pub r#version_needed: Option<super::super::types::Boolean>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#supplements: Option<super::super::types::Canonical>,
    pub r#content: super::super::types::Code,
    pub r#publisher: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#language: Option<super::super::types::Code>,
    pub r#hierarchy_meaning: Option<super::super::types::Code>,
    pub r#count: Option<super::super::types::UnsignedInt>,
    pub r#id: Option<std::string::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
}
