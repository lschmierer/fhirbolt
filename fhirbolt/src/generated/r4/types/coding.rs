// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Coding {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#user_selected: Option<super::super::types::Boolean>,
    pub r#code: Option<super::super::types::Code>,
    pub r#display: Option<super::super::types::String>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#version: Option<super::super::types::String>,
}
