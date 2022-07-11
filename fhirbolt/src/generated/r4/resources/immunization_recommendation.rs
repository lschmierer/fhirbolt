// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum ImmunizationRecommendationRecommendationDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum ImmunizationRecommendationRecommendationSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct ImmunizationRecommendationRecommendationDateCriterion {
    pub r#id: Option<std::string::String>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#value: super::super::types::DateTime,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationRecommendationRecommendation {
    pub r#date_criterion: Vec<ImmunizationRecommendationRecommendationDateCriterion>,
    pub r#forecast_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#target_disease: Option<Box<super::super::types::CodeableConcept>>,
    pub r#forecast_status: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#series: Option<super::super::types::String>,
    pub r#supporting_patient_information: Vec<Box<super::super::types::Reference>>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#vaccine_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#dose_number: Option<ImmunizationRecommendationRecommendationDoseNumber>,
    pub r#contraindicated_vaccine_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#series_doses: Option<ImmunizationRecommendationRecommendationSeriesDoses>,
    pub r#supporting_immunization: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationRecommendation {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#date: super::super::types::DateTime,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#recommendation: Vec<ImmunizationRecommendationRecommendation>,
    pub r#language: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
