// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ImagingStudySeriesPerformer {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ImagingStudySeriesInstance {
    pub r#number: Option<super::super::types::UnsignedInt>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: Option<super::super::types::String>,
    pub r#uid: super::super::types::Id,
    pub r#sop_class: Box<super::super::types::Coding>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImagingStudySeries {
    pub r#description: Option<super::super::types::String>,
    pub r#uid: super::super::types::Id,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#number: Option<super::super::types::UnsignedInt>,
    pub r#performer: Vec<ImagingStudySeriesPerformer>,
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#body_site: Option<Box<super::super::types::Coding>>,
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    pub r#instance: Vec<ImagingStudySeriesInstance>,
    pub r#laterality: Option<Box<super::super::types::Coding>>,
    pub r#modality: Box<super::super::types::Coding>,
    pub r#started: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ImagingStudy {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#number_of_series: Option<super::super::types::UnsignedInt>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#started: Option<super::super::types::DateTime>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#interpreter: Vec<Box<super::super::types::Reference>>,
    pub r#procedure_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modality: Vec<Box<super::super::types::Coding>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#description: Option<super::super::types::String>,
    pub r#procedure_reference: Option<Box<super::super::types::Reference>>,
    pub r#series: Vec<ImagingStudySeries>,
}
