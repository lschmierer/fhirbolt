// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ConsentSource {
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct ConsentPolicy {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#uri: Option<super::super::types::Uri>,
    pub r#authority: Option<super::super::types::Uri>,
}
#[derive(Debug, Clone)]
pub struct ConsentProvisionActor {
    pub r#reference: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#role: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ConsentProvisionData {
    pub r#id: Option<std::string::String>,
    pub r#meaning: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reference: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ConsentProvision {
    pub r#id: Option<std::string::String>,
    pub r#type: Option<super::super::types::Code>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#purpose: Vec<Box<super::super::types::Coding>>,
    pub r#data_period: Option<Box<super::super::types::Period>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Vec<ConsentProvisionActor>,
    pub r#provision: Vec<ConsentProvision>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Vec<Box<super::super::types::Coding>>,
    pub r#data: Vec<ConsentProvisionData>,
    pub r#action: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
}
#[derive(Debug, Clone)]
pub struct ConsentVerification {
    pub r#verified_with: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#verified: super::super::types::Boolean,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#verification_date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Consent {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#organization: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#date_time: Option<super::super::types::DateTime>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#policy: Vec<ConsentPolicy>,
    pub r#policy_rule: Option<Box<super::super::types::CodeableConcept>>,
    pub r#provision: Option<ConsentProvision>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#scope: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Option<ConsentSource>,
    pub r#language: Option<super::super::types::Code>,
    pub r#status: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#verification: Vec<ConsentVerification>,
}
