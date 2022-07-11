// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct HumanName {
    pub r#use: Option<super::super::types::Code>,
    pub r#prefix: Vec<super::super::types::String>,
    pub r#suffix: Vec<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<super::super::types::String>,
    pub r#family: Option<super::super::types::String>,
    pub r#given: Vec<super::super::types::String>,
}
