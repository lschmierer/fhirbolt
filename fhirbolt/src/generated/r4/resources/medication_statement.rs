// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum MedicationStatementMedication {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum MedicationStatementEffective {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct MedicationStatement {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#status: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status_reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#medication: MedicationStatementMedication,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#information_source: Option<Box<super::super::types::Reference>>,
    pub r#effective: Option<MedicationStatementEffective>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#date_asserted: Option<super::super::types::DateTime>,
    pub r#dosage: Vec<Box<super::super::types::Dosage>>,
}
