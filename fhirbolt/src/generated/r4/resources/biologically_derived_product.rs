// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductProcessing {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#additive: Option<Box<super::super::types::Reference>>,
    pub r#time: Option<BiologicallyDerivedProductProcessingTime>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductStorage {
    pub r#duration: Option<Box<super::super::types::Period>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#scale: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#description: Option<super::super::types::String>,
    pub r#temperature: Option<super::super::types::Decimal>,
}
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductManipulationTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductManipulation {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#time: Option<BiologicallyDerivedProductManipulationTime>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductCollection {
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#collected: Option<BiologicallyDerivedProductCollectionCollected>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#collector: Option<Box<super::super::types::Reference>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProduct {
    pub r#product_category: Option<super::super::types::Code>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#processing: Vec<BiologicallyDerivedProductProcessing>,
    pub r#status: Option<super::super::types::Code>,
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#request: Vec<Box<super::super::types::Reference>>,
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    pub r#id: Option<std::string::String>,
    pub r#storage: Vec<BiologicallyDerivedProductStorage>,
    pub r#manipulation: Option<BiologicallyDerivedProductManipulation>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#quantity: Option<super::super::types::Integer>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
}
