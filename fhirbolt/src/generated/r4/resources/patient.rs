// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct PatientContact {
    pub r#id: Option<std::string::String>,
    pub r#name: Option<Box<super::super::types::HumanName>>,
    pub r#address: Option<Box<super::super::types::Address>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
}
#[derive(Debug, Clone)]
pub enum PatientMultipleBirth {
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
}
#[derive(Debug, Clone)]
pub struct PatientLink {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#other: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct PatientCommunication {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#language: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub enum PatientDeceased {
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct Patient {
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#contact: Vec<PatientContact>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#multiple_birth: Option<PatientMultipleBirth>,
    pub r#link: Vec<PatientLink>,
    pub r#communication: Vec<PatientCommunication>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#deceased: Option<PatientDeceased>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#marital_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#general_practitioner: Vec<Box<super::super::types::Reference>>,
}
