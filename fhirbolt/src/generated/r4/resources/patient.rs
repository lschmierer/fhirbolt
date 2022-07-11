// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum PatientMultipleBirth {
    Boolean(Box<super::super::types::Boolean>),
    Integer(Box<super::super::types::Integer>),
}
#[derive(Debug, Clone)]
pub enum PatientDeceased {
    Boolean(Box<super::super::types::Boolean>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct PatientLink {
    pub r#other: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PatientCommunication {
    pub r#id: Option<std::string::String>,
    pub r#language: Box<super::super::types::CodeableConcept>,
    pub r#preferred: Option<super::super::types::Boolean>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct PatientContact {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
    pub r#name: Option<Box<super::super::types::HumanName>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#relationship: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#address: Option<Box<super::super::types::Address>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct Patient {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#multiple_birth: Option<PatientMultipleBirth>,
    pub r#language: Option<super::super::types::Code>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#deceased: Option<PatientDeceased>,
    pub r#link: Vec<PatientLink>,
    pub r#id: Option<std::string::String>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#marital_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#general_practitioner: Vec<Box<super::super::types::Reference>>,
    pub r#communication: Vec<PatientCommunication>,
    pub r#contact: Vec<PatientContact>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
