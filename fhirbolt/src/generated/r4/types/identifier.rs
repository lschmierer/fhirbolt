// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Identifier {
    pub r#value: Option<super::super::types::String>,
    pub r#use: Option<super::super::types::Code>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#assigner: Option<Box<super::super::types::Reference>>,
}
