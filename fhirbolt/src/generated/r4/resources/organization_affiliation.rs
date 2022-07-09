// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct OrganizationAffiliation {
    pub r#active: Option<super::super::types::Boolean>,
    pub r#network: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#healthcare_service: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#participating_organization: Option<Box<super::super::types::Reference>>,
    pub r#specialty: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Vec<Box<super::super::types::Reference>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
}
