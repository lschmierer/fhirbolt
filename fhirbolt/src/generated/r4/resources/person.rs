// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct PersonLink {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#assurance: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#target: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct Person {
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#link: Vec<PersonLink>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
