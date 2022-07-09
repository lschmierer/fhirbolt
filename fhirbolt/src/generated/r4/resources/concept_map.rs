// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ConceptMapSource {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupUnmapped {
    pub r#mode: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<super::super::types::Code>,
    pub r#url: Option<super::super::types::Canonical>,
    pub r#display: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupElementTargetDependsOn {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#system: Option<super::super::types::Canonical>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#display: Option<super::super::types::String>,
    pub r#property: super::super::types::Uri,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupElementTarget {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#product: Vec<ConceptMapGroupElementTargetDependsOn>,
    pub r#equivalence: super::super::types::Code,
    pub r#code: Option<super::super::types::Code>,
    pub r#display: Option<super::super::types::String>,
    pub r#comment: Option<super::super::types::String>,
    pub r#depends_on: Vec<ConceptMapGroupElementTargetDependsOn>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupElement {
    pub r#target: Vec<ConceptMapGroupElementTarget>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#display: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroup {
    pub r#id: Option<std::string::String>,
    pub r#unmapped: Option<ConceptMapGroupUnmapped>,
    pub r#element: Vec<ConceptMapGroupElement>,
    pub r#target_version: Option<super::super::types::String>,
    pub r#source_version: Option<super::super::types::String>,
    pub r#target: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Option<super::super::types::Uri>,
}
#[derive(Debug, Clone)]
pub enum ConceptMapTarget {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub struct ConceptMap {
    pub r#source: Option<ConceptMapSource>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#title: Option<super::super::types::String>,
    pub r#group: Vec<ConceptMapGroup>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#version: Option<super::super::types::String>,
    pub r#target: Option<ConceptMapTarget>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
