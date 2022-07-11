// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub enum NutritionOrderEnteralFormulaAdministrationRate {
    Quantity(Box<super::super::types::Quantity>),
    Ratio(Box<super::super::types::Ratio>),
}
#[derive(Debug, Clone)]
pub struct NutritionOrderOralDietNutrient {
    pub r#amount: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderOralDietTexture {
    pub r#id: Option<std::string::String>,
    pub r#food_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderOralDiet {
    pub r#type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#fluid_consistency_type: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
    pub r#nutrient: Vec<NutritionOrderOralDietNutrient>,
    pub r#instruction: Option<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    pub r#texture: Vec<NutritionOrderOralDietTexture>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderEnteralFormulaAdministration {
    pub r#rate: Option<NutritionOrderEnteralFormulaAdministrationRate>,
    pub r#id: Option<std::string::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#schedule: Option<Box<super::super::types::Timing>>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderEnteralFormula {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#base_formula_product_name: Option<super::super::types::String>,
    pub r#max_volume_to_deliver: Option<Box<super::super::types::Quantity>>,
    pub r#administration_instruction: Option<super::super::types::String>,
    pub r#additive_type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#additive_product_name: Option<super::super::types::String>,
    pub r#caloric_density: Option<Box<super::super::types::Quantity>>,
    pub r#routeof_administration: Option<Box<super::super::types::CodeableConcept>>,
    pub r#administration: Vec<NutritionOrderEnteralFormulaAdministration>,
    pub r#base_formula_type: Option<Box<super::super::types::CodeableConcept>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrderSupplement {
    pub r#type: Option<Box<super::super::types::CodeableConcept>>,
    pub r#schedule: Vec<Box<super::super::types::Timing>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#instruction: Option<super::super::types::String>,
    pub r#id: Option<std::string::String>,
    pub r#quantity: Option<Box<super::super::types::Quantity>>,
    pub r#product_name: Option<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub struct NutritionOrder {
    pub r#id: Option<std::string::String>,
    pub r#instantiates_canonical: Vec<super::super::types::Canonical>,
    pub r#oral_diet: Option<NutritionOrderOralDiet>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#enteral_formula: Option<NutritionOrderEnteralFormula>,
    pub r#patient: Box<super::super::types::Reference>,
    pub r#orderer: Option<Box<super::super::types::Reference>>,
    pub r#date_time: super::super::types::DateTime,
    pub r#instantiates: Vec<super::super::types::Uri>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#intent: super::super::types::Code,
    pub r#status: super::super::types::Code,
    pub r#food_preference_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#supplement: Vec<NutritionOrderSupplement>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#encounter: Option<Box<super::super::types::Reference>>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#allergy_intolerance: Vec<Box<super::super::types::Reference>>,
    pub r#note: Vec<Box<super::super::types::Annotation>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#exclude_food_modifier: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#instantiates_uri: Vec<super::super::types::Uri>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
