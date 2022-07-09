// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum TriggerDefinitionTiming {
    Timing(Box<super::super::types::Timing>),
    Reference(Box<super::super::types::Reference>),
    Date(Box<super::super::types::Date>),
    DateTime(Box<super::super::types::DateTime>),
}
#[derive(Debug, Clone)]
pub struct TriggerDefinition {
    pub r#type: super::super::types::Code,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#timing: Option<TriggerDefinitionTiming>,
    pub r#name: Option<super::super::types::String>,
    pub r#data: Vec<Box<super::super::types::DataRequirement>>,
    pub r#condition: Option<Box<super::super::types::Expression>>,
}
