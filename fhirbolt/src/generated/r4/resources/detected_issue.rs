// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DetectedIssueIdentified {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct DetectedIssueEvidence {
    pub r#detail: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DetectedIssueMitigation {
    pub r#date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#action: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct DetectedIssue {
    pub r#reference: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#severity: Option<super::super::types::Code>,
    pub r#implicated: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: super::super::types::Code,
    pub r#identified: Option<DetectedIssueIdentified>,
    pub r#evidence: Vec<DetectedIssueEvidence>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#detail: Option<super::super::types::String>,
    pub r#mitigation: Vec<DetectedIssueMitigation>,
    pub r#author: Option<Box<super::super::types::Reference>>,
}
