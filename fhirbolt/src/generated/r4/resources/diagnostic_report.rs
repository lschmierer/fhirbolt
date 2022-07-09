// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum DiagnosticReportEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct DiagnosticReportMedia {
    pub r#comment: Option<super::super::types::String>,
    pub r#link: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DiagnosticReport {
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#conclusion_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#result: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#effective: Option<DiagnosticReportEffective>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    pub r#presented_form: Vec<Box<super::super::types::Attachment>>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#results_interpreter: Vec<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#media: Vec<DiagnosticReportMedia>,
    pub r#conclusion: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#imaging_study: Vec<Box<super::super::types::Reference>>,
    pub r#issued: Option<super::super::types::Instant>,
}
