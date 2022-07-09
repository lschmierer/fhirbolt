// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum DeviceRequestParameterValue {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Quantity(Box<super::super::types::Quantity>),
    Range(Box<super::super::types::Range>),
    Boolean(Box<super::super::types::Boolean>),
}
#[derive(Debug, Clone)]
pub struct DeviceRequestParameter {
    pub r#value: Option<DeviceRequestParameterValue>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum DeviceRequestCode {
    Reference(Box<super::super::types::Reference>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub enum DeviceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub struct DeviceRequest {
    pub r#parameter: Vec<DeviceRequestParameter>,
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#language: Option<super::super::types::Code>,
    pub r#group_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#code: DeviceRequestCode,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#performer: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#status: Option<super::super::types::Code>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#occurrence: Option<DeviceRequestOccurrence>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#intent: super::super::types::Code,
    pub r#prior_request: Vec<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
