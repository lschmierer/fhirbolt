// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ImplementationGuideGlobal {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#profile: super::super::types::Canonical,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideManifestPage {
    pub r#name: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#anchor: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideManifestResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideManifestResource {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#example: Option<ImplementationGuideManifestResourceExample>,
    pub r#id: Option<std::string::String>,
    pub r#relative_path: Option<super::super::types::Url>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideManifest {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#page: Vec<ImplementationGuideManifestPage>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#resource: Vec<ImplementationGuideManifestResource>,
    pub r#image: Vec<super::super::types::String>,
    pub r#rendering: Option<super::super::types::Url>,
    pub r#other: Vec<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDependsOn {
    pub r#package_id: Option<super::super::types::Id>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#uri: super::super::types::Canonical,
    pub r#id: Option<std::string::String>,
    pub r#version: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionGrouping {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideDefinitionPageName {
    Url(Box<super::super::types::Url>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionPage {
    pub r#name: ImplementationGuideDefinitionPageName,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#generation: super::super::types::Code,
    pub r#page: Vec<ImplementationGuideDefinitionPage>,
    pub r#title: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionTemplate {
    pub r#id: Option<std::string::String>,
    pub r#source: super::super::types::String,
    pub r#code: super::super::types::Code,
    pub r#scope: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionParameter {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#value: super::super::types::String,
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideDefinitionResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionResource {
    pub r#description: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#grouping_id: Option<super::super::types::Id>,
    pub r#fhir_version: Vec<super::super::types::Code>,
    pub r#name: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#example: Option<ImplementationGuideDefinitionResourceExample>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinition {
    pub r#id: Option<std::string::String>,
    pub r#grouping: Vec<ImplementationGuideDefinitionGrouping>,
    pub r#page: Option<ImplementationGuideDefinitionPage>,
    pub r#template: Vec<ImplementationGuideDefinitionTemplate>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#parameter: Vec<ImplementationGuideDefinitionParameter>,
    pub r#resource: Vec<ImplementationGuideDefinitionResource>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuide {
    pub r#global: Vec<ImplementationGuideGlobal>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#package_id: super::super::types::Id,
    pub r#license: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#manifest: Option<ImplementationGuideManifest>,
    pub r#name: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#depends_on: Vec<ImplementationGuideDependsOn>,
    pub r#version: Option<super::super::types::String>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#title: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#fhir_version: Vec<super::super::types::Code>,
    pub r#definition: Option<ImplementationGuideDefinition>,
    pub r#status: super::super::types::Code,
    pub r#url: super::super::types::Uri,
}
