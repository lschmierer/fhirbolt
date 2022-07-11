// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct AppointmentResponse {
    pub r#end: Option<super::super::types::Instant>,
    pub r#participant_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#appointment: Box<super::super::types::Reference>,
    pub r#comment: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#start: Option<super::super::types::Instant>,
    pub r#participant_status: super::super::types::Code,
}
