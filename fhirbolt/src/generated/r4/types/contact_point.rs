// Generated on 2022-07-11 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#id: Option<std::string::String>,
    pub r#use: Option<super::super::types::Code>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#rank: Option<super::super::types::PositiveInt>,
    pub r#system: Option<super::super::types::Code>,
    pub r#value: Option<super::super::types::String>,
}
