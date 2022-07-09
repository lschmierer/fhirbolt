// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Binary {
    pub r#data: Option<super::super::types::Base64Binary>,
    pub r#content_type: super::super::types::Code,
    pub r#security_context: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
}
