// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct DocumentManifestRelated {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#ref: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct DocumentManifest {
    pub r#related: Vec<DocumentManifestRelated>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#status: super::super::types::Code,
    pub r#source: Option<super::super::types::Uri>,
    pub r#description: Option<super::super::types::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#created: Option<super::super::types::DateTime>,
    pub r#recipient: Vec<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#content: Vec<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#master_identifier: Option<Box<super::super::types::Identifier>>,
    pub r#author: Vec<Box<super::super::types::Reference>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
}
