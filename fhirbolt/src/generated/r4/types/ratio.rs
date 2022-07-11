// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Ratio {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#denominator: Option<Box<super::super::types::Quantity>>,
    pub r#numerator: Option<Box<super::super::types::Quantity>>,
}
