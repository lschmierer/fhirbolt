// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum CommunicationPayloadContent {
    String(Box<super::super::types::String>),
    Attachment(Box<super::super::types::Attachment>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct CommunicationPayload {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#content: CommunicationPayloadContent,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct Communication {
    pub r#topic: Option<Box<super::super::types::CodeableConcept>>,
    pub r#medium: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#payload: Vec<CommunicationPayload>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#in_response_to: Vec<Box<super::super::types::Reference>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#about: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#received: Option<super::super::types::DateTime>,
    pub r#sender: Option<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#sent: Option<super::super::types::DateTime>,
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
}
