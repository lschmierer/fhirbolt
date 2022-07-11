// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct OrganizationAffiliation {
    pub r#active: Option<super::super::types::Boolean>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#location: Vec<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#participating_organization: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
}
