// Generated on 2022-07-12 by fhirbolt-codegen v0.1.0
#[derive(Debug, Clone)]
pub struct DateTime {
    pub r#id: Option<std::string::String>,
    pub r#extension: Vec<Box<super::super::types::Extension>>,
    pub r#value: Option<chrono::DateTime<chrono::Utc>>,
}
