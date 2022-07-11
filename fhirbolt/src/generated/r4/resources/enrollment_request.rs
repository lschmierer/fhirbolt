// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct EnrollmentRequest {
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#id: Option<std::string::String>,
    pub r#provider: Option<Box<super::super::types::Reference>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#candidate: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#insurer: Option<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#created: Option<super::super::types::DateTime>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#coverage: Option<Box<super::super::types::Reference>>,
}
