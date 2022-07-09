// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength {
    pub r#strength: Box<super::super::types::Ratio>,
    pub r#strength_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#substance: Option<Box<super::super::types::CodeableConcept>>,
    pub r#measurement_point: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductIngredientSpecifiedSubstanceStrength {
    pub r#presentation: Box<super::super::types::Ratio>,
    pub r#country: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#reference_strength:
        Vec<MedicinalProductIngredientSpecifiedSubstanceStrengthReferenceStrength>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#concentration: Option<Box<super::super::types::Ratio>>,
    pub r#concentration_low_limit: Option<Box<super::super::types::Ratio>>,
    pub r#measurement_point: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#presentation_low_limit: Option<Box<super::super::types::Ratio>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductIngredientSpecifiedSubstance {
    pub r#group: Box<super::super::types::CodeableConcept>,
    pub r#strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#confidentiality: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductIngredientSubstance {
    pub r#code: Box<super::super::types::CodeableConcept>,
    pub r#strength: Vec<MedicinalProductIngredientSpecifiedSubstanceStrength>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct MedicinalProductIngredient {
    pub r#id: Option<std::string::String>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#allergenic_indicator: Option<super::super::types::Boolean>,
    pub r#role: Box<super::super::types::CodeableConcept>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#specified_substance: Vec<MedicinalProductIngredientSpecifiedSubstance>,
    pub r#substance: Option<MedicinalProductIngredientSubstance>,
}
