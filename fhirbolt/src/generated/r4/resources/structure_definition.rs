// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct StructureDefinitionMapping {
    pub r#name: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#uri: Option<super::super::types::Uri>,
    pub r#comment: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identity: super::super::types::Id,
}
#[derive(Debug, Clone)]
pub struct StructureDefinitionDifferential {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#element: Vec<Box<super::super::types::ElementDefinition>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct StructureDefinitionSnapshot {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#element: Vec<Box<super::super::types::ElementDefinition>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct StructureDefinitionContext {
    pub r#id: Option<std::string::String>,
    pub r#type: super::super::types::Code,
    pub r#expression: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct StructureDefinition {
    pub r#description: Option<super::super::types::Markdown>,
    pub r#derivation: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#mapping: Vec<StructureDefinitionMapping>,
    pub r#differential: Option<StructureDefinitionDifferential>,
    pub r#kind: super::super::types::Code,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#title: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#version: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#snapshot: Option<StructureDefinitionSnapshot>,
    pub r#type: super::super::types::Uri,
    pub r#base_definition: Option<super::super::types::Canonical>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#fhir_version: Option<super::super::types::Code>,
    pub r#url: super::super::types::Uri,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#abstract: super::super::types::Boolean,
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#keyword: Vec<Box<super::super::types::Coding>>,
    pub r#context: Vec<StructureDefinitionContext>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#context_invariant: Vec<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
