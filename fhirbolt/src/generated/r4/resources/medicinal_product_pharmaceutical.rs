// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#value: Box<super::super::types::Quantity>,
    pub r#supporting_information: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#tissue: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#withdrawal_period:
        Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpeciesWithdrawalPeriod>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalRouteOfAdministration {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#max_dose_per_treatment_period: Option<Box<super::super::types::Ratio>>,
    pub r#first_dose: Option<Box<super::super::types::Quantity>>,
    pub r#max_single_dose: Option<Box<super::super::types::Quantity>>,
    pub r#max_treatment_period: Option<Box<super::super::types::Duration>>,
    pub r#id: Option<std::string::String>,
    pub r#target_species: Vec<MedicinalProductPharmaceuticalRouteOfAdministrationTargetSpecies>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#max_dose_per_day: Option<Box<super::super::types::Quantity>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceuticalCharacteristics {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#code: Box<super::super::types::CodeableConcept>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductPharmaceutical {
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#route_of_administration: Vec<MedicinalProductPharmaceuticalRouteOfAdministration>,
    pub r#ingredient: Vec<Box<super::super::types::Reference>>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#characteristics: Vec<MedicinalProductPharmaceuticalCharacteristics>,
    pub r#administrable_dose_form: Box<super::super::types::CodeableConcept>,
    pub r#id: Option<std::string::String>,
}
