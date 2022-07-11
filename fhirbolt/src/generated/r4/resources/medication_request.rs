// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationRequestReported {
    Boolean(Box<super::super::types::Boolean>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationRequestSubstitutionAllowed {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub enum MedicationRequestMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicationRequestSubstitution {
    pub r#reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#allowed: MedicationRequestSubstitutionAllowed,
}
#[derive(Debug, Clone)]
pub struct MedicationRequestDispenseRequestInitialFill {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#duration: Option<Box<super::super::types::Duration>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicationRequestDispenseRequest {
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#expected_supply_duration: Option<Box<super::super::types::Duration>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#dispense_interval: Option<Box<super::super::types::Duration>>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    pub r#number_of_repeats_allowed: Option<super::super::types::UnsignedInt>,
    pub r#initial_fill: Option<MedicationRequestDispenseRequestInitialFill>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicationRequest {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#reported: Option<MedicationRequestReported>,
    pub r#event_history: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#substitution: Option<MedicationRequestSubstitution>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#recorder: Option<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    pub r#dispense_request: Option<MedicationRequestDispenseRequest>,
    pub r#detected_issue: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#prior_prescription: Option<Box<super::super::types::Reference>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#intent: super::super::types::Code,
    pub r#priority: Option<super::super::types::Code>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#dosage_instruction: Vec<Box<super::super::types::Dosage>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#medication: MedicationRequestMedication,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#course_of_therapy_type: Option<Box<super::super::types::CodeableConcept>>,
}
