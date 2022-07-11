// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct PractitionerQualification {
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#issuer: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct Practitioner {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#birth_date: Option<super::super::types::Date>,
    pub r#photo: Vec<Box<super::super::types::Attachment>>,
    pub r#communication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#address: Vec<Box<super::super::types::Address>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#qualification: Vec<PractitionerQualification>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#name: Vec<Box<super::super::types::HumanName>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
