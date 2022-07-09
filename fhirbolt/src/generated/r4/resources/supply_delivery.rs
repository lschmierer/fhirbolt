// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum SupplyDeliverySuppliedItemItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct SupplyDeliverySuppliedItem {
    pub r#item: Option<SupplyDeliverySuppliedItemItem>,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum SupplyDeliveryOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub struct SupplyDelivery {
    pub r#language: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#supplied_item: Option<SupplyDeliverySuppliedItem>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#occurrence: Option<SupplyDeliveryOccurrence>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#status: Option<super::super::types::Code>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#destination: Option<Box<super::super::types::Reference>>,
    pub r#patient: Option<Box<super::super::types::Reference>>,
    pub r#supplier: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#receiver: Vec<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
