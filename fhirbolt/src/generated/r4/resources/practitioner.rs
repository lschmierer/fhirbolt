// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct PractitionerQualification {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#issuer: Option<Box<super::super::types::Reference>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
#[derive(Debug, Clone)]
pub struct Practitioner {
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#qualification: Vec<PractitionerQualification>,
    pub r#id: Option<std::string::String>,
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
