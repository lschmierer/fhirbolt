// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum UsageContextValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct UsageContext {
    pub r#code: Box<super::super::types::Coding>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: UsageContextValue,
    pub r#id: Option<std::string::String>,
}
