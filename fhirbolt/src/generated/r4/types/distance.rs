// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Distance {
    pub r#code: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#comparator: Option<super::super::types::Code>,
    pub r#id: Option<std::string::String>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#unit: Option<super::super::types::String>,
    pub r#system: Option<super::super::types::Uri>,
}
