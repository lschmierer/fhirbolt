// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Address {
    pub r#postal_code: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#country: Option<super::super::types::String>,
    pub r#type: Option<super::super::types::Code>,
    pub r#city: Option<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#district: Option<super::super::types::String>,
    pub r#state: Option<super::super::types::String>,
    pub r#line: Vec<super::super::types::String>,
    pub r#text: Option<super::super::types::String>,
    pub r#use: Option<super::super::types::Code>,
}
