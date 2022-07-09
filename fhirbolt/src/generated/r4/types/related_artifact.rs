// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct RelatedArtifact {
    pub r#label: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: super::super::types::Code,
    pub r#document: Option<Box<super::super::types::Attachment>>,
    pub r#url: Option<super::super::types::Url>,
    pub r#resource: Option<super::super::types::Canonical>,
    pub r#display: Option<super::super::types::String>,
    pub r#citation: Option<super::super::types::Markdown>,
}
