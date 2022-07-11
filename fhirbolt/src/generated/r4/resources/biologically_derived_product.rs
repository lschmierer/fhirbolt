// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductManipulationTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductCollectionCollected {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub enum BiologicallyDerivedProductProcessingTime {
    DateTime(Box<super::super::types::DateTime>),
    Period(Box<super::super::types::Period>),
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductStorage {
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#description: Option<super::super::types::String>,
    pub r#temperature: Option<super::super::types::Decimal>,
    pub r#scale: Option<super::super::types::Code>,
    pub r#duration: Option<Box<super::super::types::Period>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductManipulation {
    pub r#description: Option<super::super::types::String>,
    pub r#time: Option<BiologicallyDerivedProductManipulationTime>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductCollection {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#source: Option<Box<super::super::types::Reference>>,
    pub r#collected: Option<BiologicallyDerivedProductCollectionCollected>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#collector: Option<Box<super::super::types::Reference>>,
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProductProcessing {
    pub r#description: Option<super::super::types::String>,
    pub r#additive: Option<Box<super::super::types::Reference>>,
    pub r#time: Option<BiologicallyDerivedProductProcessingTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#procedure: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct BiologicallyDerivedProduct {
    pub r#id: Option<std::string::String>,
    pub r#storage: Vec<BiologicallyDerivedProductStorage>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#quantity: Option<super::super::types::Integer>,
    pub r#request: Vec<Box<super::super::types::Reference>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#product_code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#parent: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#manipulation: Option<BiologicallyDerivedProductManipulation>,
    pub r#product_category: Option<super::super::types::Code>,
    pub r#status: Option<super::super::types::Code>,
    pub r#collection: Option<BiologicallyDerivedProductCollection>,
    pub r#language: Option<super::super::types::Code>,
    pub r#processing: Vec<BiologicallyDerivedProductProcessing>,
}
