// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct CodeableConcept {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#coding: Vec<Box<super::super::types::Coding>>,
    pub r#text: Option<super::super::types::String>,
}
