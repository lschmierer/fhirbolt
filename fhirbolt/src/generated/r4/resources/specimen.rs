// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
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
pub enum SpecimenProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum SpecimenContainerAdditive {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct SpecimenCollection {
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#collector: Option<Box<super::super::types::Reference>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#collected: Option<SpecimenCollectionCollected>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#fasting_status: Option<SpecimenCollectionFastingStatus>,
    pub r#duration: Option<Box<super::super::types::Duration>>,
}
#[derive(Debug, Clone)]
pub struct SpecimenProcessing {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive: Vec<Box<super::super::types::Reference>>,
    pub r#time: Option<SpecimenProcessingTime>,
}
#[derive(Debug, Clone)]
pub struct SpecimenContainer {
    pub r#additive: Option<SpecimenContainerAdditive>,
    pub r#specimen_quantity: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#capacity: Option<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub struct Specimen {
    pub r#condition: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#accession_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#collection: Option<SpecimenCollection>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#processing: Vec<SpecimenProcessing>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#container: Vec<SpecimenContainer>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#status: Option<super::super::types::Code>,
    pub r#received_time: Option<super::super::types::DateTime>,
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#request: Vec<Box<super::super::types::Reference>>,
}
