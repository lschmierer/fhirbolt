// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct CatalogEntryRelatedEntry {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#relationtype: super::super::types::Code,
    pub r#item: Box<super::super::types::Reference>,
}
#[derive(Debug, Clone)]
pub struct CatalogEntry {
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#orderable: super::super::types::Boolean,
    pub r#classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#additional_characteristic: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#referenced_item: Box<super::super::types::Reference>,
    pub r#last_updated: Option<super::super::types::DateTime>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#additional_classification: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#status: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additional_identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#validity_period: Option<Box<super::super::types::Period>>,
    pub r#valid_to: Option<super::super::types::DateTime>,
    pub r#related_entry: Vec<CatalogEntryRelatedEntry>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
}
