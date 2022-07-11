// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct OrganizationContact {
    pub r#name: Option<Box<super::super::types::HumanName>>,
    pub r#address: Option<Box<super::super::types::Address>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#purpose: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct Organization {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#name: Option<super::super::types::String>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#part_of: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#contact: Vec<OrganizationContact>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#alias: Vec<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
}
