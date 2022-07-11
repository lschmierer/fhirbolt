// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum SubstanceIngredientSubstance {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct SubstanceIngredient {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#substance: SubstanceIngredientSubstance,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Option<Box<super::super::types::Ratio>>,
}
#[derive(Debug, Clone)]
pub struct SubstanceInstance {
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#expiry: Option<super::super::types::DateTime>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct Substance {
    pub r#id: Option<std::string::String>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#category: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#description: Option<super::super::types::String>,
    pub r#status: Option<super::super::types::Code>,
    pub r#ingredient: Vec<SubstanceIngredient>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#instance: Vec<SubstanceInstance>,
}
