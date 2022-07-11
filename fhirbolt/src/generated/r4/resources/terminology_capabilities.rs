// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesImplementation {
    pub r#url: Option<super::super::types::Url>,
    pub r#description: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesTranslation {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#needs_map: super::super::types::Boolean,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesCodeSystemVersionFilter {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
    pub r#op: Vec<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesCodeSystemVersion {
    pub r#property: Vec<super::super::types::Code>,
    pub r#filter: Vec<TerminologyCapabilitiesCodeSystemVersionFilter>,
    pub r#is_default: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Vec<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#code: Option<super::super::types::String>,
    pub r#compositional: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesCodeSystem {
    pub r#id: Option<std::string::String>,
    pub r#uri: Option<super::super::types::Canonical>,
    pub r#version: Vec<TerminologyCapabilitiesCodeSystemVersion>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#subsumption: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesValidateCode {
    pub r#translations: super::super::types::Boolean,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesExpansionParameter {
    pub r#documentation: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesExpansion {
    pub r#hierarchical: Option<super::super::types::Boolean>,
    pub r#text_filter: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#incomplete: Option<super::super::types::Boolean>,
    pub r#parameter: Vec<TerminologyCapabilitiesExpansionParameter>,
    pub r#id: Option<std::string::String>,
    pub r#paging: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesClosure {
    pub r#id: Option<std::string::String>,
    pub r#translation: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilitiesSoftware {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct TerminologyCapabilities {
    pub r#implementation: Option<TerminologyCapabilitiesImplementation>,
    pub r#translation: Option<TerminologyCapabilitiesTranslation>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#code_system: Vec<TerminologyCapabilitiesCodeSystem>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#name: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version: Option<super::super::types::String>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#kind: super::super::types::Code,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#validate_code: Option<TerminologyCapabilitiesValidateCode>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#locked_date: Option<super::super::types::Boolean>,
    pub r#expansion: Option<TerminologyCapabilitiesExpansion>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#url: Option<super::super::types::Uri>,
    pub r#closure: Option<TerminologyCapabilitiesClosure>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#code_search: Option<super::super::types::Code>,
    pub r#date: super::super::types::DateTime,
    pub r#status: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#software: Option<TerminologyCapabilitiesSoftware>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#title: Option<super::super::types::String>,
}
