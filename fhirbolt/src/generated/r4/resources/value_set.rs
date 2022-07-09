// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ValueSetComposeIncludeFilter {
    pub r#property: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#op: super::super::types::Code,
    pub r#value: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ValueSetComposeIncludeConceptDesignation {
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#use: Option<Box<super::super::types::Coding>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct ValueSetComposeIncludeConcept {
    pub r#code: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#display: Option<super::super::types::String>,
    pub r#designation: Vec<ValueSetComposeIncludeConceptDesignation>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ValueSetComposeInclude {
    pub r#filter: Vec<ValueSetComposeIncludeFilter>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#value_set: Vec<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#concept: Vec<ValueSetComposeIncludeConcept>,
}
#[derive(Debug, Clone)]
pub struct ValueSetCompose {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#include: Vec<ValueSetComposeInclude>,
    pub r#exclude: Vec<ValueSetComposeInclude>,
    pub r#id: Option<std::string::String>,
    pub r#locked_date: Option<super::super::types::Date>,
    pub r#inactive: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ValueSetExpansionContains {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#abstract: Option<super::super::types::Boolean>,
    pub r#inactive: Option<super::super::types::Boolean>,
    pub r#code: Option<super::super::types::Code>,
    pub r#display: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#designation: Vec<ValueSetComposeIncludeConceptDesignation>,
    pub r#contains: Vec<ValueSetExpansionContains>,
}
#[derive(Debug, Clone)]
pub enum ValueSetExpansionParameterValue {
    String(Box<super::super::types::String>),
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
    Decimal(Box<super::super::types::Decimal>),
    Uri(Box<super::super::types::Uri>),
    Code(Box<super::super::types::Code>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct ValueSetExpansionParameter {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#value: Option<ValueSetExpansionParameterValue>,
}
#[derive(Debug, Clone)]
pub struct ValueSetExpansion {
    pub r#contains: Vec<ValueSetExpansionContains>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#total: Option<super::super::types::Integer>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#timestamp: super::super::types::DateTime,
    pub r#parameter: Vec<ValueSetExpansionParameter>,
    pub r#identifier: Option<super::super::types::Uri>,
    pub r#offset: Option<super::super::types::Integer>,
}
#[derive(Debug, Clone)]
pub struct ValueSet {
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#immutable: Option<super::super::types::Boolean>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#version: Option<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#name: Option<super::super::types::String>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#compose: Option<ValueSetCompose>,
    pub r#expansion: Option<ValueSetExpansion>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#title: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
}
