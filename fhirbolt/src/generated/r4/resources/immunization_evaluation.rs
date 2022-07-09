// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ImmunizationEvaluationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum ImmunizationEvaluationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct ImmunizationEvaluation {
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#dose_status: Box<super::super::types::CodeableConcept>,
    pub r#series: Option<super::super::types::String>,
    pub r#immunization_event: Box<super::super::types::Reference>,
    pub r#dose_number: Option<ImmunizationEvaluationDoseNumber>,
    pub r#dose_status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#series_doses: Option<ImmunizationEvaluationSeriesDoses>,
    pub r#target_disease: Box<super::super::types::CodeableConcept>,
}
