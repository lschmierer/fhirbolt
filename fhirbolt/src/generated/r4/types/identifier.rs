// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Identifier {
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#value: Option<super::super::types::String>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#use: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#assigner: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
}
