// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum ServiceRequestQuantity {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
    Range(Box<super::super::types::Range>),
}
#[derive(Debug, Clone)]
pub enum ServiceRequestOccurrence {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Timing(Box<super::super::types::Timing>),
}
#[derive(Debug, Clone)]
pub enum ServiceRequestAsNeeded {
    Boolean(Box<super::super::types::Boolean>),
    CodeableConcept(Box<super::super::types::CodeableConcept>),
}
#[derive(Debug, Clone)]
pub struct ServiceRequest {
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#requisition: Option<Box<super::super::types::Identifier>>,
    pub r#body_site: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    pub r#intent: super::super::types::Code,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#requester: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#insurance: Vec<Box<super::super::types::Reference>>,
    pub r#quantity: Option<ServiceRequestQuantity>,
    pub r#priority: Option<super::super::types::Code>,
    pub r#performer_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#location_reference: Vec<Box<super::super::types::Reference>>,
    pub r#patient_instruction: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#occurrence: Option<ServiceRequestOccurrence>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#specimen: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#relevant_history: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#order_detail: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#as_needed: Option<ServiceRequestAsNeeded>,
    pub r#status: super::super::types::Code,
    pub r#authored_on: Option<super::super::types::DateTime>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#location_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
