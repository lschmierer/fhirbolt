// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductAuthorizationJurisdictionalAuthorization {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#country: Option<Box<super::super::types::CodeableConcept>>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
}
#[derive(Debug, Clone)]
pub enum MedicinalProductAuthorizationProcedureDate {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct MedicinalProductAuthorizationProcedure {
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<MedicinalProductAuthorizationProcedureDate>,
    pub r#application: Vec<MedicinalProductAuthorizationProcedure>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductAuthorization {
    pub r#date_of_first_authorization: Option<super::super::types::DateTime>,
    pub r#jurisdictional_authorization:
        Vec<MedicinalProductAuthorizationJurisdictionalAuthorization>,
    pub r#procedure: Option<MedicinalProductAuthorizationProcedure>,
    pub r#regulator: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#international_birth_date: Option<super::super::types::DateTime>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status_date: Option<super::super::types::DateTime>,
    pub r#language: Option<super::super::types::Code>,
    pub r#jurisdiction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#data_exclusivity_period: Option<Box<super::super::types::Period>>,
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#holder: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    pub r#restore_date: Option<super::super::types::DateTime>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#legal_basis: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
