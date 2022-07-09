// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ProvenanceAgent {
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#who: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub enum ProvenanceOccurred {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct ProvenanceEntity {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#what: Box<super::super::types::Reference>,
    pub r#agent: Vec<ProvenanceAgent>,
}
#[derive(Debug, Clone)]
pub struct Provenance {
    pub r#agent: Vec<ProvenanceAgent>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#occurred: Option<ProvenanceOccurred>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#entity: Vec<ProvenanceEntity>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#id: Option<std::string::String>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#policy: Vec<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#target: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#recorded: super::super::types::Instant,
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
