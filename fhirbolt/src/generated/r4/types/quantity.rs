// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Quantity {
    pub r#id: Option<std::string::String>,
    pub r#unit: Option<super::super::types::String>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#comparator: Option<super::super::types::Code>,
    pub r#system: Option<super::super::types::Uri>,
    pub r#code: Option<super::super::types::Code>,
}
