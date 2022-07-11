// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ParameterDefinition {
    pub r#name: Option<super::super::types::Code>,
    pub r#min: Option<super::super::types::Integer>,
    pub r#max: Option<super::super::types::String>,
    pub r#type: super::super::types::Code,
    pub r#documentation: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#use: super::super::types::Code,
    pub r#profile: Option<super::super::types::Canonical>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
