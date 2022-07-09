// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum DeviceUseStatementTiming {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct DeviceUseStatement {
    pub r#language: Option<super::super::types::Code>,
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    pub r#timing: Option<DeviceUseStatementTiming>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#device: Box<super::super::types::Reference>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#status: super::super::types::Code,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#id: Option<std::string::String>,
    pub r#recorded_on: Option<super::super::types::DateTime>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
}
