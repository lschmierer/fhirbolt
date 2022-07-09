// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct MedicationBatch {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#lot_number: Option<super::super::types::String>,
    pub r#expiration_date: Option<super::super::types::DateTime>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
}
#[derive(Debug, Clone)]
pub enum MedicationIngredientItem {
    CodeableConcept(Box<super::super::types::CodeableConcept>),
    Reference(Box<super::super::types::Reference>),
}
#[derive(Debug, Clone)]
pub struct MedicationIngredient {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#item: MedicationIngredientItem,
    pub r#is_active: Option<super::super::types::Boolean>,
    pub r#id: Option<std::string::String>,
    pub r#strength: Option<Box<super::super::types::Ratio>>,
}
#[derive(Debug, Clone)]
pub struct Medication {
    pub r#status: Option<super::super::types::Code>,
    pub r#amount: Option<Box<super::super::types::Ratio>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#identifier: Vec<Box<super::super::types::Identifier>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#manufacturer: Option<Box<super::super::types::Reference>>,
    pub r#form: Option<Box<super::super::types::CodeableConcept>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#code: Option<Box<super::super::types::CodeableConcept>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#batch: Option<MedicationBatch>,
    pub r#ingredient: Vec<MedicationIngredient>,
}
