// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct PersonLink {
    pub r#target: Box<super::super::types::Reference>,
    pub r#assurance: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Person {
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#link: Vec<PersonLink>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#photo: Option<Box<super::super::types::Attachment>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
