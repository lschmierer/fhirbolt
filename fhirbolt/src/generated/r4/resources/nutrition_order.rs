// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
}
#[derive(Debug, Clone)]
pub struct NutritionOrderEnteralFormulaAdministration {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Option<Box<super::super::types::Timing>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#rate: Option<NutritionOrderEnteralFormulaAdministrationRate>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderEnteralFormula {
    pub r#administration_instruction: Option<super::super::types::String>,
    pub r#base_formula_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#caloric_density: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#base_formula_product_name: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#administration: Vec<NutritionOrderEnteralFormulaAdministration>,
    pub r#id: Option<std::string::String>,
    pub r#additive_product_name: Option<super::super::types::String>,
    pub r#routeof_administration: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderOralDietNutrient {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#amount: Option<Box<super::super::types::Quantity>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderOralDietTexture {
    pub r#id: Option<std::string::String>,
    pub r#food_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderOralDiet {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#nutrient: Vec<NutritionOrderOralDietNutrient>,
    pub r#fluid_consistency_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#instruction: Option<super::super::types::String>,
    pub r#texture: Vec<NutritionOrderOralDietTexture>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderSupplement {
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#instruction: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#product_name: Option<super::super::types::String>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrder {
    pub r#enteral_formula: Option<NutritionOrderEnteralFormula>,
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#instantiates: Vec<super::super::types::Uri>,
    pub r#date_time: super::super::types::DateTime,
    pub r#oral_diet: Option<NutritionOrderOralDiet>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#intent: super::super::types::Code,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#supplement: Vec<NutritionOrderSupplement>,
    pub r#status: super::super::types::Code,
    pub r#orderer: Option<Box<super::super::types::Reference>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#food_preference_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#allergy_intolerance: Vec<Box<super::super::types::Reference>>,
    pub r#exclude_food_modifier: Vec<Box<super::super::types::CodeableConcept>>,
}
