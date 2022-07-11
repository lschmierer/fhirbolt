// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Contributor {
    pub r#type: super::super::types::Code,
    pub r#contact: Vec<Box<super::super::types::ContactDetail>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#name: super::super::types::String,
}
