// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicinalProductSpecialDesignationIndication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicinalProductNameNamePart {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#part: super::super::types::String,
    pub r#type: Box<super::super::types::Coding>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductNameCountryLanguage {
    pub r#country: Box<super::super::types::CodeableConcept>,
    pub r#jurisdiction: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductName {
    pub r#name_part: Vec<MedicinalProductNameNamePart>,
    pub r#country_language: Vec<MedicinalProductNameCountryLanguage>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#product_name: super::super::types::String,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductSpecialDesignation {
    pub r#indication: Option<MedicinalProductSpecialDesignationIndication>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#intended_use: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#species: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductManufacturingBusinessOperation {
    pub r#authorisation_reference_number: Option<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#effective_date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#confidentiality_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#regulator: Option<Box<super::super::types::Reference>>,
    pub r#operation_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProduct {
    pub r#domain: Option<Box<super::super::types::Coding>>,
    pub r#combined_pharmaceutical_dose_form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#legal_status_of_supply: Option<Box<super::super::types::CodeableConcept>>,
    pub r#special_measures: Vec<super::super::types::String>,
    pub r#master_file: Vec<Box<super::super::types::Reference>>,
    pub r#packaged_medicinal_product: Vec<Box<super::super::types::Reference>>,
    pub r#pharmaceutical_product: Vec<Box<super::super::types::Reference>>,
    pub r#contact: Vec<Box<super::super::types::Reference>>,
    pub r#name: Vec<MedicinalProductName>,
    pub r#special_designation: Vec<MedicinalProductSpecialDesignation>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#additional_monitoring_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#cross_reference: Vec<Box<super::super::types::Identifier>>,
    pub r#marketing_status: Vec<Box<super::super::types::MarketingStatus>>,
    pub r#attached_document: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#product_classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#manufacturing_business_operation: Vec<MedicinalProductManufacturingBusinessOperation>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#paediatric_use_indicator: Option<Box<super::super::types::CodeableConcept>>,
    pub r#clinical_trial: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
