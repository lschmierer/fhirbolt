// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct EnrollmentResponse {
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#created: Option<super::super::types::DateTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#outcome: Option<super::super::types::Code>,
    pub r#disposition: Option<super::super::types::String>,
    pub r#status: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#request_provider: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#request: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#organization: Option<Box<super::super::types::Reference>>,
}
