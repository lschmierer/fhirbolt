// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ConceptMapSource {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub enum ConceptMapTarget {
    Uri(Box<super::super::types::Uri>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupUnmapped {
    pub r#display: Option<super::super::types::String>,
    pub r#mode: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#url: Option<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupElementTargetDependsOn {
    pub r#id: Option<std::string::String>,
    pub r#property: super::super::types::Uri,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#system: Option<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
    pub r#display: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupElementTarget {
    pub r#depends_on: Vec<ConceptMapGroupElementTargetDependsOn>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<super::super::types::Code>,
    pub r#display: Option<super::super::types::String>,
    pub r#product: Vec<ConceptMapGroupElementTargetDependsOn>,
    pub r#equivalence: super::super::types::Code,
    pub r#comment: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroupElement {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<super::super::types::Code>,
    pub r#display: Option<super::super::types::String>,
    pub r#target: Vec<ConceptMapGroupElementTarget>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ConceptMapGroup {
    pub r#id: Option<std::string::String>,
    pub r#target: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Option<super::super::types::Uri>,
    pub r#target_version: Option<super::super::types::String>,
    pub r#unmapped: Option<ConceptMapGroupUnmapped>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source_version: Option<super::super::types::String>,
    pub r#element: Vec<ConceptMapGroupElement>,
}
#[derive(Debug, Clone)]
pub struct ConceptMap {
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#name: Option<super::super::types::String>,
    pub r#source: Option<ConceptMapSource>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#target: Option<ConceptMapTarget>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#title: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#group: Vec<ConceptMapGroup>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#version: Option<super::super::types::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
}
