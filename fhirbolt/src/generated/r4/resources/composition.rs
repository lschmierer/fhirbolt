// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CompositionEvent {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#detail: Vec<Box<super::super::types::Reference>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CompositionAttester {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: super::super::types::Code,
    pub r#time: Option<super::super::types::DateTime>,
    pub r#party: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum CompositionRelatesToTarget {
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct CompositionRelatesTo {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#target: CompositionRelatesToTarget,
    pub r#code: super::super::types::Code,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct CompositionSection {
    pub r#title: Option<super::super::types::String>,
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#focus: Option<Box<super::super::types::Reference>>,
    pub r#section: Vec<CompositionSection>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#mode: Option<super::super::types::Code>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    pub r#entry: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Composition {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#date: super::super::types::DateTime,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#event: Vec<CompositionEvent>,
    pub r#attester: Vec<CompositionAttester>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#title: super::super::types::String,
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#relates_to: Vec<CompositionRelatesTo>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#section: Vec<CompositionSection>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#confidentiality: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
