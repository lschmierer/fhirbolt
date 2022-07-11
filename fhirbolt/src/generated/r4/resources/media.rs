// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MediaCreated {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct Media {
    pub r#created: Option<MediaCreated>,
    pub r#device_name: Option<super::super::types::String>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status: super::super::types::Code,
    pub r#duration: Option<super::super::types::Decimal>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#device: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#height: Option<super::super::types::PositiveInt>,
    pub r#view: Option<Box<super::super::types::CodeableConcept>>,
    pub r#width: Option<super::super::types::PositiveInt>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#content: Box<super::super::types::Attachment>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#operator: Option<Box<super::super::types::Reference>>,
    pub r#modality: Option<Box<super::super::types::CodeableConcept>>,
    pub r#issued: Option<super::super::types::Instant>,
    pub r#frames: Option<super::super::types::PositiveInt>,
    pub r#language: Option<super::super::types::Code>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
}
