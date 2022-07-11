// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub r#connection_type: Box<super::super::types::Coding>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: super::super::types::Code,
    pub r#contact: Vec<Box<super::super::types::ContactPoint>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#payload_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#managing_organization: Option<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#language: Option<super::super::types::Code>,
    pub r#header: Vec<super::super::types::String>,
    pub r#address: super::super::types::Url,
    pub r#name: Option<super::super::types::String>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#payload_mime_type: Vec<super::super::types::Code>,
}
