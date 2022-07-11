// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum AuditEventEntityDetailValue {
    String(Box<super::super::types::String>),
    Base64Binary(Box<super::super::types::Base64Binary>),
}
#[derive(Debug, Clone)]
pub struct AuditEventSource {
    pub r#type: Vec<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#site: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#observer: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct AuditEventEntityDetail {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::String,
    pub r#value: AuditEventEntityDetailValue,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct AuditEventEntity {
    pub r#description: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#what: Option<Box<super::super::types::Reference>>,
    pub r#role: Option<Box<super::super::types::Coding>>,
    pub r#name: Option<super::super::types::String>,
    pub r#lifecycle: Option<Box<super::super::types::Coding>>,
    pub r#query: Option<super::super::types::Base64Binary>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail: Vec<AuditEventEntityDetail>,
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    pub r#type: Option<Box<super::super::types::Coding>>,
}
#[derive(Debug, Clone)]
pub struct AuditEventAgentNetwork {
    pub r#address: Option<super::super::types::String>,
    pub r#type: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct AuditEventAgent {
    pub r#policy: Vec<super::super::types::Uri>,
    pub r#network: Option<AuditEventAgentNetwork>,
    pub r#purpose_of_use: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#alt_id: Option<super::super::types::String>,
    pub r#name: Option<super::super::types::String>,
    pub r#requestor: super::super::types::Boolean,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#media: Option<Box<super::super::types::Coding>>,
}
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub r#outcome_desc: Option<super::super::types::String>,
    pub r#purpose_of_event: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: AuditEventSource,
    pub r#entity: Vec<AuditEventEntity>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#recorded: super::super::types::Instant,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: Box<super::super::types::Coding>,
    pub r#agent: Vec<AuditEventAgent>,
    pub r#outcome: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#subtype: Vec<Box<super::super::types::Coding>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Option<super::super::types::Code>,
}
