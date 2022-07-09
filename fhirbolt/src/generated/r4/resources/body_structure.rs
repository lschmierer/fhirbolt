// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct BodyStructure {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#morphology: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#location: Option<Box<super::super::types::CodeableConcept>>,
    pub r#active: Option<super::super::types::Boolean>,
    pub r#location_qualifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#description: Option<super::super::types::String>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
}
