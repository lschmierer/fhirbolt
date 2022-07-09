// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct AppointmentResponse {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#end: Option<super::super::types::Instant>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#start: Option<super::super::types::Instant>,
    pub r#participant_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#comment: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#appointment: Box<super::super::types::Reference>,
    pub r#actor: Option<Box<super::super::types::Reference>>,
    pub r#participant_status: super::super::types::Code,
}
