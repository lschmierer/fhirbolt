// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
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
    pub r#dose_number: Option<ImmunizationEvaluationDoseNumber>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#target_disease: Box<super::super::types::CodeableConcept>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#dose_status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#description: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#series_doses: Option<ImmunizationEvaluationSeriesDoses>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#series: Option<super::super::types::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#immunization_event: Box<super::super::types::Reference>,
    pub r#dose_status: Box<super::super::types::CodeableConcept>,
}
