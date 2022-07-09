// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ParameterDefinition {
    pub r#min: Option<super::super::types::Integer>,
    pub r#use: super::super::types::Code,
    pub r#max: Option<super::super::types::String>,
    pub r#type: super::super::types::Code,
    pub r#name: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#documentation: Option<super::super::types::String>,
    pub r#profile: Option<super::super::types::Canonical>,
}
