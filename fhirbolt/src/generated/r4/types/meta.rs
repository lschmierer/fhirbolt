// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Meta {
    pub r#security: Vec<Box<super::super::types::Coding>>,
    pub r#last_updated: Option<super::super::types::Instant>,
    pub r#source: Option<super::super::types::Uri>,
    pub r#tag: Vec<Box<super::super::types::Coding>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#version_id: Option<super::super::types::Id>,
    pub r#profile: Vec<super::super::types::Canonical>,
}
