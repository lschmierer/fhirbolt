// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationDispenseMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationDispenseStatusReason {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicationDispensePerformer {
    pub r#id: Option<std::string::String>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationDispenseSubstitution {
    pub r#was_substituted: super::super::types::Boolean,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#responsible_party: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicationDispense {
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    pub r#performer: Vec<MedicationDispensePerformer>,
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    pub r#medication: MedicationDispenseMedication,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#when_prepared: Option<super::super::types::DateTime>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#receiver: Vec<Box<super::super::types::Reference>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#days_supply: Option<Box<super::super::types::Quantity>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#authorizing_prescription: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#when_handed_over: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#destination: Option<Box<super::super::types::Reference>>,
    pub r#status_reason: Option<MedicationDispenseStatusReason>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#substitution: Option<MedicationDispenseSubstitution>,
    pub r#contained: Vec<Box<super::Resource>>,
}
