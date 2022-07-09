// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum CarePlanActivityDetailScheduled {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    String(Box<super::super::types::String>),
}
#[derive(Debug, Clone)]
pub enum CarePlanActivityDetailProduct {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct CarePlanActivityDetail {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#daily_amount: Option<Box<super::super::types::Quantity>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#scheduled: Option<CarePlanActivityDetailScheduled>,
    pub r#status_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#performer: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#location: Option<Box<super::super::types::Reference>>,
    pub r#do_not_perform: Option<super::super::types::Boolean>,
    pub r#goal: Vec<Box<super::super::types::Reference>>,
    pub r#product: Option<CarePlanActivityDetailProduct>,
    pub r#kind: Option<super::super::types::Code>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct CarePlanActivity {
    pub r#progress: Vec<Box<super::super::types::Annotation>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#outcome_codeable_concept: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#reference: Option<Box<super::super::types::Reference>>,
    pub r#detail: Option<CarePlanActivityDetail>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#outcome_reference: Vec<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct CarePlan {
    pub r#language: Option<super::super::types::Code>,
    pub r#activity: Vec<CarePlanActivity>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#supporting_info: Vec<Box<super::super::types::Reference>>,
    pub r#part_of: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#title: Option<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#care_team: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#description: Option<super::super::types::String>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#replaces: Vec<Box<super::super::types::Reference>>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#intent: super::super::types::Code,
    pub r#addresses: Vec<Box<super::super::types::Reference>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#author: Option<Box<super::super::types::Reference>>,
    pub r#contributor: Vec<Box<super::super::types::Reference>>,
    pub r#created: Option<super::super::types::DateTime>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#goal: Vec<Box<super::super::types::Reference>>,
}
