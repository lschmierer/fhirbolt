// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Binary {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#content_type: super::super::types::Code,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#security_context: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#data: Option<super::super::types::Base64Binary>,
}
