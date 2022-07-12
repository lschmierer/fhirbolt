// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct Time {
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<chrono::naive::NaiveTime>,
    pub r#id: Option<std::string::String>,
}
