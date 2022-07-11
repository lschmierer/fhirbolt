// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ProcedurePerformed {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
    Age(Box<super::super::types::Age>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub struct ProcedureFocalDevice {
    pub r#id: Option<std::string::String>,
    pub r#action: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#manipulated: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ProcedurePerformer {
    pub r#actor: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Procedure {
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#used_reference: Vec<Box<super::super::types::Reference>>,
    pub r#focal_device: Vec<ProcedureFocalDevice>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#complication: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#performed: Option<ProcedurePerformed>,
    pub r#performer: Vec<ProcedurePerformer>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#follow_up: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#complication_detail: Vec<Box<super::super::types::Reference>>,
    pub r#report: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#used_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
}
