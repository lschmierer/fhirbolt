// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ProductShelfLife {
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#identifier: Option<Box<super::super::types::Identifier>>,
    pub r#type: Box<super::super::types::CodeableConcept>,
    pub r#period: Box<super::super::types::Quantity>,
    pub r#special_precautions_for_storage: Vec<Box<super::super::types::CodeableConcept>>,
    pub r#id: Option<std::string::String>,
}
