// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct RelatedPersonCommunication {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Box<super::super::types::CodeableConcept>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct RelatedPerson {
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#communication: Vec<RelatedPersonCommunication>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
}
