// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct EpisodeOfCareStatusHistory {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Box<super::super::types::Period>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct EpisodeOfCareDiagnosis {
    pub r#role: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#condition: Box<super::super::types::Reference>,
    pub r#rank: Option<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub struct EpisodeOfCare {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#status_history: Vec<EpisodeOfCareStatusHistory>,
    pub r#team: Vec<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#diagnosis: Vec<EpisodeOfCareDiagnosis>,
    pub r#care_manager: Option<Box<super::super::types::Reference>>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#referral_request: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#account: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
}
