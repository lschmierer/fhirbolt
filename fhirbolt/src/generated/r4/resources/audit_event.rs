// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct AuditEventSource {
    pub r#site: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::Coding>>,
    pub r#observer: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum AuditEventEntityDetailValue {
    String(Box<super::super::types::String>),
    Base64Binary(Box<super::super::types::Base64Binary>),
}
#[derive(Debug, Clone)]
pub struct AuditEventEntityDetail {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#value: AuditEventEntityDetailValue,
    pub r#type: super::super::types::String,
}
#[derive(Debug, Clone)]
pub struct AuditEventEntity {
    pub r#query: Option<super::super::types::Base64Binary>,
    pub r#what: Option<Box<super::super::types::Reference>>,
    pub r#role: Option<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#detail: Vec<AuditEventEntityDetail>,
    pub r#type: Option<Box<super::super::types::Coding>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#name: Option<super::super::types::String>,
    pub r#lifecycle: Option<Box<super::super::types::Coding>>,
    pub r#security_label: Vec<Box<super::super::types::Coding>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct AuditEventAgentNetwork {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<super::super::types::Code>,
    pub r#address: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct AuditEventAgent {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#name: Option<super::super::types::String>,
    pub r#network: Option<AuditEventAgentNetwork>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#requestor: super::super::types::Boolean,
    pub r#policy: Vec<super::super::types::Uri>,
    pub r#purpose_of_use: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#who: Option<Box<super::super::types::Reference>>,
    pub r#role: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#media: Option<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#alt_id: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct AuditEvent {
    pub r#source: AuditEventSource,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#subtype: Vec<Box<super::super::types::Coding>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#recorded: super::super::types::Instant,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#entity: Vec<AuditEventEntity>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Box<super::super::types::Coding>,
    pub r#agent: Vec<AuditEventAgent>,
    pub r#action: Option<super::super::types::Code>,
    pub r#outcome: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#outcome_desc: Option<super::super::types::String>,
    pub r#purpose_of_event: Vec<Box<super::super::types::CodeableConcept>>,
}
