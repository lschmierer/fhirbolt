// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct DataRequirementSort {
    pub r#id: Option<std::string::String>,
    pub r#path: super::super::types::String,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#direction: super::super::types::Code,
}
#[derive(Debug, Clone)]
pub enum DataRequirementSubject {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub enum DataRequirementDateFilterValue {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
    Duration(Box<super::super::types::Duration>),
}
#[derive(Debug, Clone)]
pub struct DataRequirementDateFilter {
    pub r#path: Option<super::super::types::String>,
    pub r#value: Option<DataRequirementDateFilterValue>,
    pub r#id: Option<std::string::String>,
    pub r#search_param: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DataRequirementCodeFilter {
    pub r#path: Option<super::super::types::String>,
    pub r#code: Vec<Box<super::super::types::Coding>>,
    pub r#search_param: Option<super::super::types::String>,
    pub r#value_set: Option<super::super::types::Canonical>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct DataRequirement {
    pub r#type: super::super::types::Code,
    pub r#sort: Vec<Box<super::super::types::Element>>,
    pub r#subject: Option<DataRequirementSubject>,
    pub r#limit: Option<super::super::types::PositiveInt>,
    pub r#id: Option<std::string::String>,
    pub r#date_filter: Vec<Box<super::super::types::Element>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#profile: Vec<super::super::types::Canonical>,
    pub r#must_support: Vec<super::super::types::String>,
    pub r#code_filter: Vec<Box<super::super::types::Element>>,
}
