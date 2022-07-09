// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Signature {
    pub r#who: Box<super::super::types::Reference>,
    pub r#target_format: Option<super::super::types::Code>,
    pub r#on_behalf_of: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Vec<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#data: Option<super::super::types::Base64Binary>,
    pub r#sig_format: Option<super::super::types::Code>,
    pub r#when: super::super::types::Instant,
}
