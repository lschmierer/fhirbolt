// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ChargeItemProduct {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct ChargeItemPerformer {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#function: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#actor: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub enum ChargeItemOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub struct ChargeItem {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#performing_organization: Option<Box<super::super::types::Reference>>,
    pub r#price_override: Option<Box<super::super::types::Money>>,
    pub r#product: Option<ChargeItemProduct>,
    pub r#context: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#override_reason: Option<super::super::types::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#performer: Vec<ChargeItemPerformer>,
    pub r#bodysite: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#service: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#cost_center: Option<Box<super::super::types::Reference>>,
    pub r#definition_uri: Vec<super::super::types::Uri>,
    pub r#entered_date: Option<super::super::types::DateTime>,
    pub r#account: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#requesting_organization: Option<Box<super::super::types::Reference>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#factor_override: Option<super::super::types::Decimal>,
    pub r#reason: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#supporting_information: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#definition_canonical: Vec<super::super::types::Canonical>,
    pub r#enterer: Option<Box<super::super::types::Reference>>,
    pub r#occurrence: Option<ChargeItemOccurrence>,
}
