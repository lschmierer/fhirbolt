// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct DocumentReferenceRelatesTo {
    pub r#code: super::super::types::Code,
    pub r#target: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DocumentReferenceContent {
    pub r#format: Option<Box<super::super::types::Coding>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#attachment: Box<super::super::types::Attachment>,
}
#[derive(Debug, Clone)]
pub struct DocumentReferenceContext {
    pub r#event: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#facility_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Vec<Box<super::super::types::Reference>>,
    pub r#practice_setting: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#source_patient_info: Option<Box<super::super::types::Reference>>,
    pub r#related: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DocumentReference {
    pub r#security_label: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#master_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#authenticator: Option<Box<super::super::types::Reference>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#relates_to: Vec<DocumentReferenceRelatesTo>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#content: Vec<DocumentReferenceContent>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#doc_status: Option<super::super::types::Code>,
    pub r#date: Option<super::super::types::Instant>,
    pub r#description: Option<super::super::types::String>,
    pub r#context: Option<DocumentReferenceContext>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#custodian: Option<Box<super::super::types::Reference>>,
}
