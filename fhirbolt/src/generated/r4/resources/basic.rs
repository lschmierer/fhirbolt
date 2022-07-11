// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Basic {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#created: Option<super::super::types::Date>,
    pub r#id: Option<std::string::String>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
