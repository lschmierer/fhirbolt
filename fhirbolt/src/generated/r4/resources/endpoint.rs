// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#payload_mime_type: Vec<super::super::types::Code>,
    pub r#connection_type: Box<super::super::types::Coding>,
    pub r#header: Vec<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#address: super::super::types::Url,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#payload_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#name: Option<super::super::types::String>,
}
