// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Flag {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: super::super::types::Code,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
