// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct RelatedPersonCommunication {
    pub r#language: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct RelatedPerson {
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#communication: Vec<RelatedPersonCommunication>,
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
