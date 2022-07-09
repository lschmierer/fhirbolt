// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CodeableConcept {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#coding: Vec<Box<super::super::types::Coding>>,
    pub r#text: Option<super::super::types::String>,
}
