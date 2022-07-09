// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ProdCharacteristic {
    pub r#depth: Option<Box<super::super::types::Quantity>>,
    pub r#weight: Option<Box<super::super::types::Quantity>>,
    pub r#width: Option<Box<super::super::types::Quantity>>,
    pub r#external_diameter: Option<Box<super::super::types::Quantity>>,
    pub r#shape: Option<super::super::types::String>,
    pub r#imprint: Vec<super::super::types::String>,
    pub r#modifier_extension: Vec<Box<super::super::types::Extension>>,
    pub r#height: Option<Box<super::super::types::Quantity>>,
    pub r#color: Vec<super::super::types::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#image: Vec<Box<super::super::types::Attachment>>,
    pub r#nominal_volume: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#scoring: Option<Box<super::super::types::CodeableConcept>>,
}
