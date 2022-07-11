// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ImagingStudySeriesPerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImagingStudySeriesInstance {
    pub r#title: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#uid: super::super::types::Id,
    pub r#sop_class: Box<super::super::types::Coding>,
    pub r#number: Option<super::super::types::UnsignedInt>,
}
#[derive(Debug, Clone)]
pub struct ImagingStudySeries {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    pub r#uid: super::super::types::Id,
    pub r#body_site: Option<Box<super::super::types::Coding>>,
    pub r#laterality: Option<Box<super::super::types::Coding>>,
    pub r#started: Option<super::super::types::DateTime>,
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    pub r#number: Option<super::super::types::UnsignedInt>,
    pub r#performer: Vec<ImagingStudySeriesPerformer>,
    pub r#description: Option<super::super::types::String>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#modality: Box<super::super::types::Coding>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#instance: Vec<ImagingStudySeriesInstance>,
}
#[derive(Debug, Clone)]
pub struct ImagingStudy {
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#procedure_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#number_of_series: Option<super::super::types::UnsignedInt>,
    pub r#number_of_instances: Option<super::super::types::UnsignedInt>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#description: Option<super::super::types::String>,
    pub r#series: Vec<ImagingStudySeries>,
    pub r#interpreter: Vec<Box<super::super::types::Reference>>,
    pub r#endpoint: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#started: Option<super::super::types::DateTime>,
    pub r#modality: Vec<Box<super::super::types::Coding>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#status: super::super::types::Code,
    pub r#referrer: Option<Box<super::super::types::Reference>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#procedure_reference: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
}
