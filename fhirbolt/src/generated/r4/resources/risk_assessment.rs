// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum RiskAssessmentPredictionWhen {
    Period(Box<super::super::types::Period>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub enum RiskAssessmentPredictionProbability {
    Decimal(Box<super::super::types::Decimal>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub struct RiskAssessmentPrediction {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#outcome: Option<Box<super::super::types::CodeableConcept>>,
    pub r#relative_risk: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#qualitative_risk: Option<Box<super::super::types::CodeableConcept>>,
    pub r#when: Option<RiskAssessmentPredictionWhen>,
    pub r#id: Option<std::string::String>,
    pub r#rationale: Option<super::super::types::String>,
    pub r#probability: Option<RiskAssessmentPredictionProbability>,
}
#[derive(Debug, Clone)]
pub enum RiskAssessmentOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct RiskAssessment {
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#parent: Option<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#prediction: Vec<RiskAssessmentPrediction>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#condition: Option<Box<super::super::types::Reference>>,
    pub r#basis: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#occurrence: Option<RiskAssessmentOccurrence>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#based_on: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#mitigation: Option<super::super::types::String>,
}
