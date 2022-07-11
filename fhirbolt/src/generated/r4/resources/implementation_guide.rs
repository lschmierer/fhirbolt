// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ImplementationGuideDefinitionPageName {
    Url(Box<super::super::types::Url>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideDefinitionResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub enum ImplementationGuideManifestResourceExample {
    Boolean(Box<super::super::types::Boolean>),
    Canonical(Box<super::super::types::Canonical>),
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideGlobal {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#profile: super::super::types::Canonical,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionParameter {
    pub r#code: super::super::types::Code,
    pub r#value: super::super::types::String,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionGrouping {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionPage {
    pub r#page: Vec<ImplementationGuideDefinitionPage>,
    pub r#generation: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#title: super::super::types::String,
    pub r#name: ImplementationGuideDefinitionPageName,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionTemplate {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#source: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#scope: Option<super::super::types::String>,
    pub r#code: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinitionResource {
    pub r#reference: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#fhir_version: Vec<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#example: Option<ImplementationGuideDefinitionResourceExample>,
    pub r#grouping_id: Option<super::super::types::Id>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDefinition {
    pub r#parameter: Vec<ImplementationGuideDefinitionParameter>,
    pub r#grouping: Vec<ImplementationGuideDefinitionGrouping>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#page: Option<ImplementationGuideDefinitionPage>,
    pub r#template: Vec<ImplementationGuideDefinitionTemplate>,
    pub r#resource: Vec<ImplementationGuideDefinitionResource>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideManifestResource {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#relative_path: Option<super::super::types::Url>,
    pub r#id: Option<std::string::String>,
    pub r#example: Option<ImplementationGuideManifestResourceExample>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideManifestPage {
    pub r#name: super::super::types::String,
    pub r#title: Option<super::super::types::String>,
    pub r#anchor: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideManifest {
    pub r#id: Option<std::string::String>,
    pub r#resource: Vec<ImplementationGuideManifestResource>,
    pub r#image: Vec<super::super::types::String>,
    pub r#page: Vec<ImplementationGuideManifestPage>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#other: Vec<super::super::types::String>,
    pub r#rendering: Option<super::super::types::Url>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuideDependsOn {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#package_id: Option<super::super::types::Id>,
    pub r#version: Option<super::super::types::String>,
    pub r#uri: super::super::types::Canonical,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImplementationGuide {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#global: Vec<ImplementationGuideGlobal>,
    pub r#definition: Option<ImplementationGuideDefinition>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#fhir_version: Vec<super::super::types::Code>,
    pub r#url: super::super::types::Uri,
    pub r#license: Option<super::super::types::Code>,
    pub r#title: Option<super::super::types::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#package_id: super::super::types::Id,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#manifest: Option<ImplementationGuideManifest>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#depends_on: Vec<ImplementationGuideDependsOn>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#version: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#status: super::super::types::Code,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#name: super::super::types::String,
}
