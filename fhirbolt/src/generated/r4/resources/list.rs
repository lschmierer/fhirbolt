// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ListEntry {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#deleted: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#item: Box<super::super::types::Reference>,
    pub r#flag: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct List {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#mode: super::super::types::Code,
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#title: Option<super::super::types::String>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#entry: Vec<ListEntry>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#status: super::super::types::Code,
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
}
