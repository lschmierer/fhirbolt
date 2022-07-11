// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Address {
    pub r#use: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<super::super::types::Code>,
    pub r#text: Option<super::super::types::String>,
    pub r#line: Vec<super::super::types::String>,
    pub r#state: Option<super::super::types::String>,
    pub r#city: Option<super::super::types::String>,
    pub r#postal_code: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#country: Option<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#district: Option<super::super::types::String>,
}
