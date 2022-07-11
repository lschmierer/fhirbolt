// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ProvenanceOccurred {
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct ProvenanceAgent {
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#who: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct ProvenanceEntity {
    pub r#role: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#what: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#agent: Vec<ProvenanceAgent>,
}
#[derive(Debug, Clone)]
pub struct Provenance {
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#activity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#agent: Vec<ProvenanceAgent>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#occurred: Option<ProvenanceOccurred>,
    pub r#policy: Vec<super::super::types::Uri>,
    pub r#entity: Vec<ProvenanceEntity>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#target: Vec<Box<super::super::types::Reference>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#recorded: super::super::types::Instant,
    pub r#signature: Vec<Box<super::super::types::Signature>>,
}
