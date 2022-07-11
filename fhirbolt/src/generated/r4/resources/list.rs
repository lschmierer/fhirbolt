// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ListEntry {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#flag: Option<Box<super::super::types::CodeableConcept>>,
    pub r#item: Box<super::super::types::Reference>,
    pub r#deleted: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct List {
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#subject: Option<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#mode: super::super::types::Code,
    pub r#entry: Vec<ListEntry>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#status: super::super::types::Code,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#empty_reason: Option<Box<super::super::types::CodeableConcept>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#date: Option<super::super::types::DateTime>,
    pub r#ordered_by: Option<Box<super::super::types::CodeableConcept>>,
    pub r#title: Option<super::super::types::String>,
}
