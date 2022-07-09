// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct SubstanceInstance {
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#expiry: Option<super::super::types::DateTime>,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum SubstanceIngredientSubstance {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct SubstanceIngredient {
    pub r#quantity: Option<Box<super::super::types::Ratio>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: SubstanceIngredientSubstance,
}
#[derive(Debug, Clone)]
pub struct Substance {
    pub r#language: Option<super::super::types::Code>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#status: Option<super::super::types::Code>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#instance: Vec<SubstanceInstance>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#id: Option<std::string::String>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#ingredient: Vec<SubstanceIngredient>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#description: Option<super::super::types::String>,
}
