// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct OperationDefinitionParameterReferencedFrom {
    pub r#source_id: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct OperationDefinitionParameterBinding {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#strength: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#value_set: super::super::types::Canonical,
}
#[derive(Debug, Clone)]
pub struct OperationDefinitionParameter {
    pub r#name: super::super::types::Code,
    pub r#target_profile: Vec<super::super::types::Canonical>,
    pub r#use: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#referenced_from: Vec<OperationDefinitionParameterReferencedFrom>,
    pub r#min: super::super::types::Integer,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#documentation: Option<super::super::types::String>,
    pub r#part: Vec<OperationDefinitionParameter>,
    pub r#binding: Option<OperationDefinitionParameterBinding>,
    pub r#max: super::super::types::String,
    pub r#type: Option<super::super::types::Code>,
    pub r#search_type: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct OperationDefinitionOverload {
    pub r#comment: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#parameter_name: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct OperationDefinition {
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#type: super::super::types::Boolean,
    pub r#instance: super::super::types::Boolean,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#parameter: Vec<OperationDefinitionParameter>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#system: super::super::types::Boolean,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#overload: Vec<OperationDefinitionOverload>,
    pub r#output_profile: Option<super::super::types::Canonical>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#kind: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#input_profile: Option<super::super::types::Canonical>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#code: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#version: Option<super::super::types::String>,
    pub r#title: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#comment: Option<super::super::types::Markdown>,
    pub r#resource: Vec<super::super::types::Code>,
    pub r#base: Option<super::super::types::Canonical>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#affects_state: Option<super::super::types::Boolean>,
    pub r#name: super::super::types::String,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
}
