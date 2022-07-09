// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    pub r#tissue: Box<super::super::types::CodeableConcept>,
    pub r#supporting_information: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Box<super::super::types::Quantity>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#withdrawal_period:
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    pub r#target_species: Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceutical {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#administrable_dose_form: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#ingredient: Vec<Box<super::super::types::Reference>>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#characteristics: Vec<MedicinalProductPharmaceuticalCharacteristics>,
    pub r#route_of_administration: Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
}
