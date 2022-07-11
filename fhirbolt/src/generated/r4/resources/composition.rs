// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum CompositionRelatesToTarget {
    Identifier(Box<super::super::types::Identifier>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct CompositionRelatesTo {
    pub r#code: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#target: CompositionRelatesToTarget,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CompositionEvent {
    pub r#detail: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
}
#[derive(Debug, Clone)]
pub struct CompositionAttester {
    pub r#id: Option<std::string::String>,
    pub r#mode: super::super::types::Code,
    pub r#party: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#time: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CompositionSection {
    pub r#mode: Option<super::super::types::Code>,
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    pub r#entry: Vec<Box<super::super::types::Reference>>,
    pub r#focus: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: Option<super::super::types::String>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#section: Vec<CompositionSection>,
    pub r#id: Option<std::string::String>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Composition {
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#date: super::super::types::DateTime,
    pub r#custodian: Option<Box<super::super::types::Reference>>,
    pub r#relates_to: Vec<CompositionRelatesTo>,
    pub r#event: Vec<CompositionEvent>,
    pub r#attester: Vec<CompositionAttester>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#confidentiality: Option<super::super::types::Code>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#title: super::super::types::String,
    pub r#section: Vec<CompositionSection>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
}
