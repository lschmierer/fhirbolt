// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct Age {
    pub r#id: Option<std::string::String>,
    pub r#comparator: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#unit: Option<super::super::types::String>,
    pub r#code: Option<super::super::types::Code>,
    pub r#value: Option<super::super::types::Decimal>,
    pub r#system: Option<super::super::types::Uri>,
}
