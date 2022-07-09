// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct EpisodeOfCareDiagnosis {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#condition: Box<super::super::types::Reference>,
    pub r#rank: Option<super::super::types::PositiveInt>,
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct EpisodeOfCareStatusHistory {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Box<super::super::types::Period>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub struct EpisodeOfCare {
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#diagnosis: Vec<EpisodeOfCareDiagnosis>,
    pub r#language: Option<super::super::types::Code>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#care_manager: Option<Box<super::super::types::Reference>>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#referral_request: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#status_history: Vec<EpisodeOfCareStatusHistory>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#account: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#team: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
