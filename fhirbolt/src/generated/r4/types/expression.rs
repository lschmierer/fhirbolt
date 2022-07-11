// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Expression {
    pub r#language: super::super::types::Code,
    pub r#name: Option<super::super::types::Id>,
    pub r#description: Option<super::super::types::String>,
    pub r#reference: Option<super::super::types::Uri>,
    pub r#id: Option<std::string::String>,
    pub r#expression: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
