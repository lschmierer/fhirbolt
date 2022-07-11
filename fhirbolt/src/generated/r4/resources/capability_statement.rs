// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct CapabilityStatementMessagingEndpoint {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#protocol: Box<super::super::types::Coding>,
    pub r#address: super::super::types::Url,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementMessagingSupportedMessage {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#definition: super::super::types::Canonical,
    pub r#mode: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementMessaging {
    pub r#reliable_cache: Option<super::super::types::UnsignedInt>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#endpoint: Vec<CapabilityStatementMessagingEndpoint>,
    pub r#supported_message: Vec<CapabilityStatementMessagingSupportedMessage>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRestResourceSearchParam {
    pub r#name: super::super::types::String,
    pub r#type: super::super::types::Code,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#definition: Option<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRestResourceOperation {
    pub r#id: Option<std::string::String>,
    pub r#definition: super::super::types::Canonical,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#name: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRestResourceInteraction {
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRestResource {
    pub r#reference_policy: Vec<super::super::types::Code>,
    pub r#type: super::super::types::Code,
    pub r#supported_profile: Vec<super::super::types::Canonical>,
    pub r#conditional_update: Option<super::super::types::Boolean>,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#versioning: Option<super::super::types::Code>,
    pub r#search_rev_include: Vec<super::super::types::String>,
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    pub r#conditional_create: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#read_history: Option<super::super::types::Boolean>,
    pub r#update_create: Option<super::super::types::Boolean>,
    pub r#conditional_delete: Option<super::super::types::Code>,
    pub r#search_include: Vec<super::super::types::String>,
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
    pub r#interaction: Vec<CapabilityStatementRestResourceInteraction>,
    pub r#conditional_read: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRestInteraction {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#code: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRestSecurity {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#cors: Option<super::super::types::Boolean>,
    pub r#service: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::Markdown>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementRest {
    pub r#resource: Vec<CapabilityStatementRestResource>,
    pub r#id: Option<std::string::String>,
    pub r#compartment: Vec<super::super::types::Canonical>,
    pub r#interaction: Vec<CapabilityStatementRestInteraction>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#search_param: Vec<CapabilityStatementRestResourceSearchParam>,
    pub r#security: Option<CapabilityStatementRestSecurity>,
    pub r#mode: super::super::types::Code,
    pub r#operation: Vec<CapabilityStatementRestResourceOperation>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementDocument {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#documentation: Option<super::super::types::Markdown>,
    pub r#id: Option<std::string::String>,
    pub r#profile: super::super::types::Canonical,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementImplementation {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    pub r#description: super::super::types::String,
    pub r#url: Option<super::super::types::Url>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatementSoftware {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#release_date: Option<super::super::types::DateTime>,
    pub r#version: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct CapabilityStatement {
    pub r#url: Option<super::super::types::Uri>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#messaging: Vec<CapabilityStatementMessaging>,
    pub r#experimental: Option<super::super::types::Boolean>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#kind: super::super::types::Code,
    pub r#rest: Vec<CapabilityStatementRest>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#document: Vec<CapabilityStatementDocument>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#status: super::super::types::Code,
    pub r#patch_format: Vec<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#version: Option<super::super::types::String>,
    pub r#imports: Vec<super::super::types::Canonical>,
    pub r#name: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#implementation: Option<CapabilityStatementImplementation>,
    pub r#implementation_guide: Vec<super::super::types::Canonical>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#purpose: Option<super::super::types::Markdown>,
    pub r#instantiates: Vec<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#fhir_version: super::super::types::Code,
    pub r#software: Option<CapabilityStatementSoftware>,
    pub r#format: Vec<super::super::types::Code>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#copyright: Option<super::super::types::Markdown>,
    pub r#title: Option<super::super::types::String>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#date: super::super::types::DateTime,
}
