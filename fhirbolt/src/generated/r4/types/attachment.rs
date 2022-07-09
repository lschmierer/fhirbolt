// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Attachment {
    pub r#id: Option<std::string::String>,
    pub r#url: Option<super::super::types::Url>,
    pub r#creation: Option<super::super::types::DateTime>,
    pub r#data: Option<super::super::types::Base64Binary>,
    pub r#hash: Option<super::super::types::Base64Binary>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#title: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#size: Option<super::super::types::UnsignedInt>,
    pub r#content_type: Option<super::super::types::Code>,
}
