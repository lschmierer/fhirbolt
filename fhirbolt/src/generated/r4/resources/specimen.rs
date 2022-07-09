// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum SpecimenCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum SpecimenCollectionFastingStatus {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Duration(Box<super::super::types::Duration>),
}
#[derive(Debug, Clone)]
pub struct SpecimenCollection {
    pub r#duration: Option<Box<super::super::types::Duration>>,
    pub r#collector: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#collected: Option<SpecimenCollectionCollected>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#fasting_status: Option<SpecimenCollectionFastingStatus>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum SpecimenProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct SpecimenProcessing {
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#time: Option<SpecimenProcessingTime>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub enum SpecimenContainerAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct SpecimenContainer {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive: Option<SpecimenContainerAdditive>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
    pub r#specimen_quantity: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
#[derive(Debug, Clone)]
pub struct Specimen {
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#collection: Option<SpecimenCollection>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#received_time: Option<super::super::types::DateTime>,
    pub r#processing: Vec<SpecimenProcessing>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    pub r#container: Vec<SpecimenContainer>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#accession_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#request: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
