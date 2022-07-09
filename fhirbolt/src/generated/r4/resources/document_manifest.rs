// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct DocumentManifestRelated {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#ref: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct DocumentManifest {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#language: Option<super::super::types::Code>,
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
    pub r#master_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#content: Vec<Box<super::super::types::Reference>>,
    pub r#related: Vec<DocumentManifestRelated>,
    pub r#created: Option<super::super::types::DateTime>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#source: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#description: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
