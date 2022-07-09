// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Basic {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#created: Option<super::super::types::Date>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
}
