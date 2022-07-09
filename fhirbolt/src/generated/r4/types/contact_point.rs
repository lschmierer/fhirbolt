// Generated on 2022-07-09 by fhirbolt-codegen version 0.1.0
#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub r#value: Option<super::super::types::String>,
    pub r#period: Option<Box<super::super::types::Period>>,
    pub r#system: Option<super::super::types::Code>,
    pub r#rank: Option<super::super::types::PositiveInt>,
    pub r#id: Option<std::string::String>,
    pub r#use: Option<super::super::types::Code>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
}
