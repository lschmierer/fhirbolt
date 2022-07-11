// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Money {
    pub r#value: Option<super::super::types::Decimal>,
    pub r#id: Option<std::string::String>,
    pub r#currency: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
