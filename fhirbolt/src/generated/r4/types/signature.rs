// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Signature {
    pub r#when: super::super::types::Instant,
    pub r#target_format: Option<super::super::types::Code>,
    pub r#data: Option<super::super::types::Base64Binary>,
    pub r#type: Vec<Box<super::super::types::Coding>>,
    pub r#sig_format: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#who: Box<super::super::types::Reference>,
}
