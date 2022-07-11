// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Reference {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<super::super::types::Uri>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#reference: Option<super::super::types::String>,
    pub r#display: Option<super::super::types::String>,
}
