// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SupplyRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub enum SupplyRequestParameterValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
}
#[derive(Debug, Clone)]
pub enum SupplyRequestItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct SupplyRequestParameter {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<SupplyRequestParameterValue>,
}
#[derive(Debug, Clone)]
pub struct SupplyRequest {
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#language: Option<super::super::types::Code>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#deliver_from: Option<Box<super::super::types::Reference>>,
    pub r#occurrence: Option<SupplyRequestOccurrence>,
    pub r#category: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#parameter: Vec<SupplyRequestParameter>,
    pub r#id: Option<std::string::String>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#supplier: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#item: SupplyRequestItem,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#deliver_to: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
