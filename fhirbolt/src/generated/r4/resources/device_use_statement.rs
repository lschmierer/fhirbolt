// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum DeviceUseStatementTiming {
    Timing(Box<super::super::types::Timing>),
    Period(Box<super::super::types::Period>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct DeviceUseStatement {
    pub r#recorded_on: Option<super::super::types::DateTime>,
    pub r#device: Box<super::super::types::Reference>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#body_site: Option<Box<super::super::types::CodeableConcept>>,
    pub r#status: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#reason_code: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#reason_reference: Vec<Box<super::super::types::Reference>>,
    pub r#derived_from: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#based_on: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Box<super::super::types::Reference>,
    pub r#timing: Option<DeviceUseStatementTiming>,
}
