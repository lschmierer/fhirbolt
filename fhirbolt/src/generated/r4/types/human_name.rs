// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct HumanName {
    pub r#prefix: Vec<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#text: Option<super::super::types::String>,
    pub r#use: Option<super::super::types::Code>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#given: Vec<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#suffix: Vec<super::super::types::String>,
    pub r#family: Option<super::super::types::String>,
}
