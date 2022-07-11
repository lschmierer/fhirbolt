// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
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
pub enum ImmunizationOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub struct ImmunizationProtocolApplied {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#authority: Option<Box<super::super::types::Reference>>,
    pub r#target_disease: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#dose_number: ImmunizationProtocolAppliedDoseNumber,
    pub r#id: Option<std::string::String>,
    pub r#series: Option<super::super::types::String>,
    pub r#series_doses: Option<ImmunizationProtocolAppliedSeriesDoses>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationEducation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#document_type: Option<super::super::types::String>,
    pub r#reference: Option<super::super::types::Uri>,
    pub r#publication_date: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#presentation_date: Option<super::super::types::DateTime>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationPerformer {
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct ImmunizationReaction {
    pub r#reported: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#detail: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
}
#[derive(Debug, Clone)]
pub struct Immunization {
    pub r#language: Option<super::super::types::Code>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#lot_number: Option<super::super::types::String>,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#subpotent_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#recorded: Option<super::super::types::DateTime>,
    pub r#protocol_applied: Vec<ImmunizationProtocolApplied>,
    pub r#expiration_date: Option<super::super::types::Date>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#education: Vec<ImmunizationEducation>,
    pub r#site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#id: Option<std::string::String>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#funding_source: Option<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#report_origin: Option<Box<super::super::types::CodeableConcept>>,
    pub r#route: Option<Box<super::super::types::CodeableConcept>>,
    pub r#performer: Vec<ImmunizationPerformer>,
    pub r#dose_quantity: Option<Box<super::super::types::Quantity>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#vaccine_code: Box<super::super::types::CodeableConcept>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#is_subpotent: Option<super::super::types::Boolean>,
    pub r#program_eligibility: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#primary_source: Option<super::super::types::Boolean>,
    pub r#reaction: Vec<ImmunizationReaction>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#occurrence: ImmunizationOccurrence,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
}
