// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ImmunizationProtocolAppliedDoseNumber {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum ImmunizationProtocolAppliedSeriesDoses {
    PositiveInt(Box<super::super::types::PositiveInt>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct ImmunizationProtocolApplied {
    pub r#dose_number: ImmunizationProtocolAppliedDoseNumber,
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#series_doses: Option<ImmunizationProtocolAppliedSeriesDoses>,
    pub r#series: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#target_disease: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationPerformer {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationEducation {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#publication_date: Option<super::super::types::DateTime>,
    pub r#presentation_date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#document_type: Option<super::super::types::String>,
    pub r#reference: Option<super::super::types::Uri>,
}
#[derive(Debug, Clone)]
pub enum ImmunizationOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct ImmunizationReaction {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#detail: Option<Box<super::super::types::Reference>>,
    pub r#reported: Option<super::super::types::Boolean>,
}
#[derive(Debug, Clone)]
pub struct Immunization {
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#protocol_applied: Vec<ImmunizationProtocolApplied>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#report_origin: Option<Box<super::super::types::CodeableConcept>>,
    pub r#recorded: Option<super::super::types::DateTime>,
    pub r#expiration_date: Option<super::super::types::Date>,
    pub r#performer: Vec<ImmunizationPerformer>,
    pub r#education: Vec<ImmunizationEducation>,
    pub r#program_eligibility: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#occurrence: ImmunizationOccurrence,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#dose_quantity: Option<Box<super::super::types::Quantity>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#is_subpotent: Option<super::super::types::Boolean>,
    pub r#subpotent_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#vaccine_code: Box<super::super::types::CodeableConcept>,
    pub r#funding_source: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#reaction: Vec<ImmunizationReaction>,
    pub r#status: super::super::types::Code,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#primary_source: Option<super::super::types::Boolean>,
    pub r#lot_number: Option<super::super::types::String>,
}
