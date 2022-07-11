// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct MedicinalProductManufactured {
    pub r#physical_characteristics: Option<Box<super::super::types::ProdCharacteristic>>,
    pub r#implicit_rules: Option<super::super::types::Uri>,
    pub r#manufacturer: Vec<Box<super::super::types::Reference>>,
    pub r#language: Option<super::super::types::Code>,
    pub r#meta: Option<Box<super::super::types::Meta>>,
    pub r#text: Option<Box<super::super::types::Narrative>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#quantity: Box<super::super::types::Quantity>,
    pub r#manufactured_dose_form: Box<super::super::types::CodeableConcept>,
    pub r#other_characteristics: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#ingredient: Vec<Box<super::super::types::Reference>>,
    pub r#contained: Vec<Box<super::Resource>>,
    pub r#unit_of_presentation: Option<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
