// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CareTeamParticipant {
    pub r#member: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct CareTeam {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#name: Option<super::super::types::String>,
    pub r#participant: Vec<CareTeamParticipant>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#managing_organization: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
