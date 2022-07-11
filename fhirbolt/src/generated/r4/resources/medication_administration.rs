// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationAdministrationMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationAdministrationDosageRate {
    Ratio(Box<super::super::types::Ratio>),
    Quantity(Box<super::super::types::Quantity>),
}
#[derive(Debug, Clone)]
pub enum MedicationAdministrationEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct MedicationAdministrationPerformer {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationAdministrationDosage {
    pub r#dose: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#method: Option<Box<super::super::types::CodeableConcept>>,
    pub r#rate: Option<MedicationAdministrationDosageRate>,
}
#[derive(Debug, Clone)]
pub struct MedicationAdministration {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#medication: MedicationAdministrationMedication,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#performer: Vec<MedicationAdministrationPerformer>,
    pub r#device: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#dosage: Option<MedicationAdministrationDosage>,
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#effective: MedicationAdministrationEffective,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates: Vec<super::super::types::Uri>,
}
