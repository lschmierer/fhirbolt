// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct CatalogEntryRelatedEntry {
    pub r#id: Option<std::string::String>,
    pub r#relationtype: super::super::types::Code,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: Box<super::super::types::Reference>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub r#id: Option<std::string::String>,
    pub r#related_entry: Vec<CatalogEntryRelatedEntry>,
    pub r#last_updated: Option<super::super::types::DateTime>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#orderable: super::super::types::Boolean,
    pub r#additional_identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#additional_characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#valid_to: Option<super::super::types::DateTime>,
    pub r#additional_classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#referenced_item: Box<super::super::types::Reference>,
}
