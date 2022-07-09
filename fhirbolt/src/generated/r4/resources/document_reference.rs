// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct DocumentReferenceRelatesTo {
    pub r#target: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DocumentReferenceContent {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#attachment: Box<super::super::types::Attachment>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#format: Option<Box<super::super::types::Coding>>,
}
#[derive(Debug, Clone)]
pub struct DocumentReferenceContext {
    pub r#id: Option<std::string::String>,
    pub r#event: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#facility_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#source_patient_info: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#practice_setting: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#related: Vec<Box<super::super::types::Reference>>,
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct DocumentReference {
    pub r#id: Option<std::string::String>,
    pub r#master_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#doc_status: Option<super::super::types::Code>,
    pub r#relates_to: Vec<DocumentReferenceRelatesTo>,
    pub r#security_label: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    pub r#content: Vec<DocumentReferenceContent>,
    pub r#description: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status: super::super::types::Code,
    pub r#context: Option<DocumentReferenceContext>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#authenticator: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::Instant>,
}
