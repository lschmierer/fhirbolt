// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct NamingSystemUniqueId {
    pub r#id: Option<std::string::String>,
    pub r#type: super::super::types::Code,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#value: super::super::types::String,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#comment: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct NamingSystem {
    pub r#unique_id: Vec<NamingSystemUniqueId>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#publisher: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#kind: super::super::types::Code,
    pub r#date: super::super::types::DateTime,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#usage: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#use_context: Vec<Box<super::super::types::UsageContext>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#responsible: Option<super::super::types::String>,
    pub r#description: Option<super::super::types::Markdown>,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
