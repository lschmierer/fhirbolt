// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct RelatedArtifact {
    pub r#citation: Option<super::super::types::Markdown>,
    pub r#document: Option<Box<super::super::types::Attachment>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#url: Option<super::super::types::Url>,
    pub r#id: Option<std::string::String>,
    pub r#display: Option<super::super::types::String>,
    pub r#label: Option<super::super::types::String>,
    pub r#type: super::super::types::Code,
    pub r#resource: Option<super::super::types::Canonical>,
}
