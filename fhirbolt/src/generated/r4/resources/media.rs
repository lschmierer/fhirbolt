// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum MediaCreated {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct Media {
    pub r#created: Option<MediaCreated>,
    pub r#content: Box<super::super::types::Attachment>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#duration: Option<super::super::types::Decimal>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#height: Option<super::super::types::PositiveInt>,
    pub r#id: Option<std::string::String>,
    pub r#device_name: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#operator: Option<Box<super::super::types::Reference>>,
    pub r#view: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#frames: Option<super::super::types::PositiveInt>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#width: Option<super::super::types::PositiveInt>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#issued: Option<super::super::types::Instant>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#modality: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
