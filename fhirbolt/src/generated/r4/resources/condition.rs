// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ConditionOnset {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum ConditionAbatement {
    DateTime(Box<super::super::types::DateTime>),
    Age(Box<super::super::types::Age>),
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct ConditionEvidence {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#detail: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ConditionStage {
    pub r#id: Option<std::string::String>,
    pub r#assessment: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#summary: Option<Box<super::super::types::CodeableConcept>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Condition {
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#onset: Option<ConditionOnset>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#abatement: Option<ConditionAbatement>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#evidence: Vec<ConditionEvidence>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#stage: Vec<ConditionStage>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
