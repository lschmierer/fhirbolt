// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Range {
    pub r#low: Option<Box<super::super::types::Quantity>>,
    pub r#id: Option<std::string::String>,
    pub r#high: Option<Box<super::super::types::Quantity>>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
