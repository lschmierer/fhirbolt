// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct OperationDefinitionOverload {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#parameter_name: Vec<super::super::types::String>,
    pub r#comment: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct OperationDefinitionParameterReferencedFrom {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source_id: Option<super::super::types::String>,
    pub r#source: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct OperationDefinitionParameterBinding {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value_set: super::super::types::Canonical,
    pub r#strength: super::super::types::Code,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct OperationDefinitionParameter {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#max: super::super::types::String,
    pub r#documentation: Option<super::super::types::String>,
    pub r#type: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#search_type: Option<super::super::types::Code>,
    pub r#target_profile: Vec<super::super::types::Canonical>,
    pub r#referenced_from: Vec<OperationDefinitionParameterReferencedFrom>,
    pub r#part: Vec<OperationDefinitionParameter>,
    pub r#name: super::super::types::Code,
    pub r#use: super::super::types::Code,
    pub r#min: super::super::types::Integer,
    pub r#binding: Option<OperationDefinitionParameterBinding>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct OperationDefinition {
    pub r#output_profile: Option<super::super::types::Canonical>,
    pub r#title: Option<super::super::types::String>,
    pub r#instance: super::super::types::Boolean,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#overload: Vec<OperationDefinitionOverload>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#code: super::super::types::Code,
    pub r#version: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#comment: Option<super::super::types::Markdown>,
    pub r#name: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#affects_state: Option<super::super::types::Boolean>,
    pub r#parameter: Vec<OperationDefinitionParameter>,
    pub r#id: Option<std::string::String>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#system: super::super::types::Boolean,
    pub r#input_profile: Option<super::super::types::Canonical>,
    pub r#kind: super::super::types::Code,
    pub r#publisher: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#type: super::super::types::Boolean,
    pub r#url: Option<super::super::types::Uri>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#resource: Vec<super::super::types::Code>,
    pub r#base: Option<super::super::types::Canonical>,
}
