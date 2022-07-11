// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ObservationDefinitionQualifiedInterval {
    pub r#context: Option<Box<super::super::types::CodeableConcept>>,
    pub r#applies_to: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#gender: Option<super::super::types::Code>,
    pub r#condition: Option<super::super::types::String>,
    pub r#gestational_age: Option<Box<super::super::types::Range>>,
    pub r#age: Option<Box<super::super::types::Range>>,
    pub r#range: Option<Box<super::super::types::Range>>,
    pub r#category: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ObservationDefinitionQuantitativeDetails {
    pub r#unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#conversion_factor: Option<super::super::types::Decimal>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#decimal_precision: Option<super::super::types::Integer>,
    pub r#id: Option<std::string::String>,
    pub r#customary_unit: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ObservationDefinition {
    pub r#normal_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#qualified_interval: Vec<ObservationDefinitionQualifiedInterval>,
    pub r#id: Option<std::string::String>,
    pub r#preferred_report_name: Option<super::super::types::String>,
    pub r#valid_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#multiple_results_allowed: Option<super::super::types::Boolean>,
    pub r#critical_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#quantitative_details: Option<ObservationDefinitionQuantitativeDetails>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#abnormal_coded_value_set: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#permitted_data_type: Vec<super::super::types::Code>,
}
