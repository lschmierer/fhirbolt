// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum DosageDoseAndRateRate {
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub enum DosageDoseAndRateDose {
    Range(Box<super::super::types::Range>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub struct DosageDoseAndRate {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#rate: Option<DosageDoseAndRateRate>,
    pub r#dose: Option<DosageDoseAndRateDose>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub enum DosageAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct Dosage {
    pub r#max_dose_per_period: Option<Box<super::super::types::Ratio>>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<super::super::types::String>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#dose_and_rate: Vec<Box<super::super::types::Element>>,
    pub r#max_dose_per_lifetime: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#sequence: Option<super::super::types::Integer>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#additional_instruction: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#as_needed: Option<DosageAsNeeded>,
    pub r#patient_instruction: Option<super::super::types::String>,
    pub r#max_dose_per_administration: Option<Box<super::super::types::Quantity>>,
    pub r#timing: Option<Box<super::super::types::Timing>>,
}
