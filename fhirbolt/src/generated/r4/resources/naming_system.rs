// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct NamingSystemUniqueId {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#type: super::super::types::Code,
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[derive(Debug, Clone)]
pub struct NamingSystem {
    pub r#language: Option<super::super::types::Code>,
    pub r#usage: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: super::super::types::String,
    pub r#kind: super::super::types::Code,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#responsible: Option<super::super::types::String>,
    pub r#date: super::super::types::DateTime,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#publisher: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#unique_id: Vec<NamingSystemUniqueId>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
}
