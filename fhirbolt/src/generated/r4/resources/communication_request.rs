// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum CommunicationRequestPayloadContent {
    String(Box<super::super::types::String>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum CommunicationRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct CommunicationRequestPayload {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: CommunicationRequestPayloadContent,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CommunicationRequest {
    pub r#sender: Option<Box<super::super::types::Reference>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#about: Vec<Box<super::super::types::Reference>>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#payload: Vec<CommunicationRequestPayload>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#occurrence: Option<CommunicationRequestOccurrence>,
    pub r#medium: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
}
