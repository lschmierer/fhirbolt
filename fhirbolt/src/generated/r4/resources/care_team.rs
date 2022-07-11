// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct CareTeamParticipant {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#member: Option<Box<super::super::types::Reference>>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[derive(Debug, Clone)]
pub struct CareTeam {
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#managing_organization: Vec<Box<super::super::types::Reference>>,
    pub r#telecom: Vec<Box<super::super::types::ContactPoint>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#participant: Vec<CareTeamParticipant>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
}
