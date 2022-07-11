// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DosageAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub enum DosageDoseAndRateDose {
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub enum DosageDoseAndRateRate {
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub struct DosageDoseAndRate {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#dose: Option<DosageDoseAndRateDose>,
    pub r#rate: Option<DosageDoseAndRateRate>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct Dosage {
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#max_dose_per_period: Option<Box<super::super::types::Ratio>>,
    pub r#as_needed: Option<DosageAsNeeded>,
    pub r#patient_instruction: Option<super::super::types::String>,
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#timing: Option<Box<super::super::types::Timing>>,
    pub r#dose_and_rate: Vec<Box<super::super::types::Element>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#sequence: Option<super::super::types::Integer>,
    pub r#text: Option<super::super::types::String>,
    pub r#additional_instruction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
}
