// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ConsentVerification {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#verification_date: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#verified: super::super::types::Boolean,
    pub r#verified_with: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum ConsentSource {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ConsentPolicy {
    pub r#id: Option<std::string::String>,
    pub r#authority: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#uri: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ConsentProvisionData {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meaning: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct ConsentProvisionActor {
    pub r#reference: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct ConsentProvision {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Vec<Box<super::super::types::Coding>>,
    pub r#provision: Vec<ConsentProvision>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#data_period: Option<Box<super::super::types::Period>>,
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    pub r#data: Vec<ConsentProvisionData>,
    pub r#purpose: Vec<Box<super::super::types::Coding>>,
    pub r#actor: Vec<ConsentProvisionActor>,
    pub r#action: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Consent {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#verification: Vec<ConsentVerification>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#scope: Box<super::super::types::CodeableConcept>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#organization: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#source: Option<ConsentSource>,
    pub r#policy: Vec<ConsentPolicy>,
    pub r#policy_rule: Option<Box<super::super::types::CodeableConcept>>,
    pub r#provision: Option<ConsentProvision>,
    pub r#date_time: Option<super::super::types::DateTime>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
}
