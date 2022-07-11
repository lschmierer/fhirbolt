// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
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
pub struct ConditionStage {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#assessment: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#summary: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct ConditionEvidence {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#detail: Vec<Box<super::super::types::Reference>>,
    pub r#code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Condition {
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#stage: Vec<ConditionStage>,
    pub r#onset: Option<ConditionOnset>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#evidence: Vec<ConditionEvidence>,
    pub r#language: Option<super::super::types::Code>,
    pub r#clinical_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#abatement: Option<ConditionAbatement>,
    pub r#recorded_date: Option<super::super::types::DateTime>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#asserter: Option<Box<super::super::types::Reference>>,
    pub r#verification_status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#severity: Option<Box<super::super::types::CodeableConcept>>,
}
