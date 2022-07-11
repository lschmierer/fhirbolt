// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct EncounterLocation {
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#location: Box<super::super::types::Reference>,
    pub r#physical_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: Option<super::super::types::Code>,
}
#[derive(Debug, Clone)]
pub struct EncounterDiagnosis {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#condition: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#use: Option<Box<super::super::types::CodeableConcept>>,
    pub r#rank: Option<super::super::types::PositiveInt>,
}
#[derive(Debug, Clone)]
pub struct EncounterHospitalization {
    pub r#re_admission: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#special_courtesy: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#pre_admission_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#admit_source: Option<Box<super::super::types::CodeableConcept>>,
    pub r#special_arrangement: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#origin: Option<Box<super::super::types::Reference>>,
    pub r#diet_preference: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#destination: Option<Box<super::super::types::Reference>>,
    pub r#discharge_disposition: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct EncounterClassHistory {
    pub r#period: Box<super::super::types::Period>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#class: Box<super::super::types::Coding>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct EncounterParticipant {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#individual: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct EncounterStatusHistory {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Box<super::super::types::Period>,
}
#[derive(Debug, Clone)]
pub struct Encounter {
    pub r#appointment: Vec<Box<super::super::types::Reference>>,
    pub r#location: Vec<EncounterLocation>,
    pub r#service_provider: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#account: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#diagnosis: Vec<EncounterDiagnosis>,
    pub r#hospitalization: Option<EncounterHospitalization>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#class: Box<super::super::types::Coding>,
    pub r#length: Option<Box<super::super::types::Duration>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#class_history: Vec<EncounterClassHistory>,
    pub r#service_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#participant: Vec<EncounterParticipant>,
    pub r#status_history: Vec<EncounterStatusHistory>,
    pub r#priority: Option<Box<super::super::types::CodeableConcept>>,
    pub r#episode_of_care: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Option<Box<super::super::types::Reference>>,
}
